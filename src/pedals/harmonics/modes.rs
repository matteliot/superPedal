//! The four modes, all running, crossfaded on a switch:
//!   1 the input
//!   2 harmonics only: a notch on the fundamental, then a 4th-order high-pass at 1.5 f0
//!   3 low harmonics: resonators on harmonics 2..=4
//!   4 high harmonics: notches on h4 and h3, then a 4th-order high-pass at 4.5 f0
//! The filters follow the tracked pitch, retuned every 32 samples.

use super::presets::*;
use crate::pedalkit::dsp::{Biquad, coef, db};

pub struct Modes {
    m2_bp: Biquad,
    m2_hp: [Biquad; 2],
    m3_bp: [Biquad; 3],
    m4_bp: Biquad,
    m4_bp3: Biquad,
    m4_hp: [Biquad; 2],
    /// Each mode's crossfade gain.
    pub gains: [f32; 4],
    makeup: [f32; 4],
    retune: u32,
    c_xfade: f32,
}

impl Modes {
    pub const NEW: Self = Self {
        m2_bp: Biquad::NEW,
        m2_hp: [Biquad::NEW; 2],
        m3_bp: [Biquad::NEW; 3],
        m4_bp: Biquad::NEW,
        m4_bp3: Biquad::NEW,
        m4_hp: [Biquad::NEW; 2],
        gains: [1.0, 0.0, 0.0, 0.0],
        makeup: [1.0; 4],
        retune: 0,
        c_xfade: 1.0,
    };

    /// `f` the starting pitch, `mode` the starting mode (heard at once).
    pub fn init(&mut self, f: f32, mode: usize) {
        self.tune(f);
        for (m, d) in self.makeup.iter_mut().zip(MAKEUP_DB) {
            *m = db(d);
        }
        self.c_xfade = coef(XFADE_S);
        self.gains = [0.0; 4];
        self.gains[mode] = 1.0;
    }

    /// Point the mode filters at pitch `f`.
    fn tune(&mut self, f: f32) {
        let lim = |x: f32| x.min(F_MAX);
        self.m2_bp.bandpass(lim(f), FUND_Q);
        self.m2_hp[0].highpass(lim(M2_HP * f), 0.541);
        self.m2_hp[1].highpass(lim(M2_HP * f), 1.307);
        for (k, bp) in self.m3_bp.iter_mut().enumerate() {
            bp.bandpass(lim((k as f32 + 2.0) * f), LOW_Q);
        }
        self.m4_bp.bandpass(lim(4.0 * f), NOTCH4_Q);
        self.m4_bp3.bandpass(lim(3.0 * f), NOTCH4_Q);
        self.m4_hp[0].highpass(lim(M4_HP * f), 0.541);
        self.m4_hp[1].highpass(lim(M4_HP * f), 1.307);
    }

    /// `x` the guitar, `f` its tracked pitch, `mode` the one to fade to.
    #[inline]
    pub fn process(&mut self, x: f32, f: f32, mode: usize) -> f32 {
        self.retune = self.retune.wrapping_add(1);
        if self.retune & 31 == 0 {
            self.tune(f);
        }
        let m2 = x - self.m2_bp.process(x);
        let m2 = self.m2_hp[0].process(m2);
        let m2 = self.m2_hp[1].process(m2);
        let m3 = self.m3_bp.iter_mut().map(|bp| bp.process(x)).sum::<f32>();
        let m4 = x - self.m4_bp.process(x);
        let m4 = m4 - self.m4_bp3.process(m4);
        let m4 = self.m4_hp[0].process(m4);
        let m4 = self.m4_hp[1].process(m4);
        let modes = [x, m2, m3, m4];
        let mut y = 0.0;
        for k in 0..4 {
            let t = if k == mode { 1.0 } else { 0.0 };
            self.gains[k] += (t - self.gains[k]) * self.c_xfade;
            y += self.gains[k] * self.makeup[k] * modes[k];
        }
        y
    }
}
