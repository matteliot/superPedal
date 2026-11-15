//! Tone Bender MkII: germanium, soft and asymmetric, then a biased second
//! stage that sputters as notes die; tight lows, a thick mid hump, soft top.

use super::presets::{TB_BIAS, TB_GAIN_DB, TB_OUT};
use crate::pedalkit::dsp::{Biquad, db, tanh};
use crate::pedalkit::helpers::biased;

pub struct ToneBender {
    hp: Biquad,
    lp: Biquad,
    mid: Biquad,
}

impl ToneBender {
    pub const NEW: Self = Self { hp: Biquad::NEW, lp: Biquad::NEW, mid: Biquad::NEW };

    pub fn init(&mut self) {
        self.hp.highpass(110.0, 0.707);
        self.lp.lowpass(4500.0, 0.707);
        self.mid.peak(800.0, 0.8, 3.0);
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let u = self.hp.process(x) * db(TB_GAIN_DB);
        let a = if u > 0.0 { tanh(u) } else { tanh(0.6 * u) * 1.2 };
        let b = biased(2.0 * a, TB_BIAS);
        self.mid.process(self.lp.process(b)) * TB_OUT
    }
}
