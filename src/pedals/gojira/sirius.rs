//! Sirius voice: a tight boost -> a two-stage modern high-gain preamp -> a
//! deep, slightly scooped tone stack -> power-amp squash (mono).

use super::presets::*;
use crate::pedalkit::dsp::{Biquad, db, tanh};
use crate::pedalkit::helpers::{Clip2x, biased};

pub struct Sirius {
    boost_hp: Biquad,
    boost_lp: Biquad,
    pre_hp: Biquad,
    clip1: Clip2x,
    inter_hp: Biquad,
    inter_lp: Biquad,
    clip2: Clip2x,
    tone_low: Biquad,
    tone_mid: Biquad,
    tone_pres: Biquad,
    tone_dc: Biquad,
    tone_lp: Biquad,
}

impl Sirius {
    pub const NEW: Self = Self {
        boost_hp: Biquad::NEW,
        boost_lp: Biquad::NEW,
        pre_hp: Biquad::NEW,
        clip1: Clip2x::NEW,
        inter_hp: Biquad::NEW,
        inter_lp: Biquad::NEW,
        clip2: Clip2x::NEW,
        tone_low: Biquad::NEW,
        tone_mid: Biquad::NEW,
        tone_pres: Biquad::NEW,
        tone_dc: Biquad::NEW,
        tone_lp: Biquad::NEW,
    };

    pub fn init(&mut self) {
        // A tight boost (mid hump, lows left alone) ...
        self.boost_hp.highpass(BOOST_HP, 0.5);
        self.boost_lp.lowpass(5000.0, 0.707);
        // ... into a modern high-gain preamp, tightened again at its input.
        self.pre_hp.highpass(100.0, 0.707);
        self.inter_hp.highpass(140.0, 0.707); // also strips stage 1's DC
        self.inter_lp.lowpass(6000.0, 0.707);
        // Deep low end, mids pulled back a little, crisp presence.
        self.tone_low.peak(90.0, 0.9, 4.0);
        self.tone_mid.peak(480.0, 0.9, -4.5);
        self.tone_pres.peak(3200.0, 0.9, 4.0);
        self.tone_dc.highpass(30.0, 0.707); // stage 2's DC
        self.tone_lp.lowpass(7500.0, 0.707);
    }

    /// `g` the preamp's stage 1 gain.
    #[inline]
    pub fn process(&mut self, x: f32, g: f32) -> f32 {
        // Tight boost, used the way metal players use a Tube Screamer: low
        // drive, and only the mids and up are clipped, so the low end stays
        // tight and the amp gets a mid hump to bite on.
        let x = x + tanh(self.boost_hp.process(x) * BOOST_DRIVE) * BOOST_CLIP;
        let x = self.pre_hp.process(self.boost_lp.process(x));
        let y = self.clip1.process(x * g, |u| biased(u, BIAS1));
        let y = self.inter_lp.process(self.inter_hp.process(y));
        let y = self.clip2.process(y * db(GAIN2_DB), |u| biased(u, BIAS2));
        let y = self.tone_pres.process(self.tone_mid.process(self.tone_low.process(y)));
        let y = self.tone_lp.process(self.tone_dc.process(y));
        tanh(y * POWER_DRIVE) * (1.0 / POWER_DRIVE)
    }
}
