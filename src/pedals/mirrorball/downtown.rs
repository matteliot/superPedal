//! Downtown voice: a tweed's two preamp stages -> tweed tone -> a 6V6 power
//! amp on a sagging rectifier (mono). Loose lows (a tweed doesn't tighten
//! anything up), a little low-mid wool, a mid push and a chiming top (the
//! Firebird pickup's bite, which keeps open chords clear).

use super::presets::*;
use crate::pedalkit::dsp::{Biquad, db};
use crate::pedalkit::env::Follower;
use crate::pedalkit::helpers::{Clip2x, biased};

pub struct Downtown {
    pre_hp: Biquad,
    clip1: Clip2x,
    inter_hp: Biquad,
    inter_lp: Biquad,
    clip2: Clip2x,
    tone_low: Biquad,
    tone_mid: Biquad,
    tone_treb: Biquad,
    tone_dc: Biquad,
    sag: Follower,
}

impl Downtown {
    pub const NEW: Self = Self {
        pre_hp: Biquad::NEW,
        clip1: Clip2x::NEW,
        inter_hp: Biquad::NEW,
        inter_lp: Biquad::NEW,
        clip2: Clip2x::NEW,
        tone_low: Biquad::NEW,
        tone_mid: Biquad::NEW,
        tone_treb: Biquad::NEW,
        tone_dc: Biquad::NEW,
        sag: Follower::NEW,
    };

    pub fn init(&mut self) {
        self.pre_hp.highpass(60.0, 0.707);
        self.inter_hp.highpass(40.0, 0.707); // also strips stage 1's DC
        self.inter_lp.lowpass(7000.0, 0.707);
        self.tone_low.peak(180.0, 0.8, 1.0);
        self.tone_mid.peak(900.0, 0.7, 3.0);
        self.tone_treb.peak(3000.0, 0.8, 3.5);
        self.tone_dc.highpass(30.0, 0.707);
        self.sag.times(SAG_ATT_S, SAG_REL_S);
    }

    /// `g` stage 1's gain (the volume knob on 10).
    #[inline]
    pub fn process(&mut self, x: f32, g: f32) -> f32 {
        let x = self.pre_hp.process(x);
        let y = self.clip1.process(x * g, |u| biased(u, BIAS1));
        let y = self.inter_lp.process(self.inter_hp.process(y));
        let y = self.clip2.process(y * db(DRIVE2_DB), |u| biased(u, BIAS2));
        let y = self.tone_treb.process(self.tone_mid.process(self.tone_low.process(y)));
        let y = self.tone_dc.process(y);
        // Power amp on a sagging rectifier: a hard hit pulls the supply
        // down, squashing the attack, and the note swells back as it recovers.
        let s = self.sag.follow(y.abs());
        let p = y * POWER_DRIVE / (1.0 + SAG * s);
        biased(p, POWER_BIAS) * (1.0 / POWER_DRIVE) * 1.5
    }
}
