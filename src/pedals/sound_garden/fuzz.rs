//! Two fuzz stages, each with cheap 2x oversampling: a honky pre-mid push
//! into a biased tanh, band-limited between, then a second tanh.

use super::presets::ASYM_BIAS;
use crate::pedalkit::dsp::{Biquad, tanh};
use crate::pedalkit::helpers::{Clip2x, biased};

pub struct Fuzz {
    pre_mid: Biquad,
    clip1: Clip2x,
    inter_lp: Biquad,
    inter_hp: Biquad,
    clip2: Clip2x,
}

impl Fuzz {
    pub const NEW: Self =
        Self { pre_mid: Biquad::NEW, clip1: Clip2x::NEW, inter_lp: Biquad::NEW, inter_hp: Biquad::NEW, clip2: Clip2x::NEW };

    pub fn init(&mut self) {
        self.pre_mid.peak(900.0, 0.7, 5.0); // Guild-ish honk
        self.inter_lp.lowpass(5500.0, 0.707);
        self.inter_hp.highpass(70.0, 0.707); // also removes the asym clipper's DC
    }

    /// `g1`, `g2`: the two stages' gains.
    #[inline]
    pub fn process(&mut self, x: f32, g1: f32, g2: f32) -> f32 {
        let u = self.pre_mid.process(x) * g1;
        let y = self.clip1.process(u, |u| biased(u, ASYM_BIAS));
        let y = self.inter_hp.process(self.inter_lp.process(y));
        self.clip2.process(y * g2, tanh)
    }
}
