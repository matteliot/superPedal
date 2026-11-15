//! Peace voice: the same tweed at a low volume (mono).

use super::presets::{BIAS1, PEACE_DRIVE_DB};
use crate::pedalkit::dsp::{Biquad, db};
use crate::pedalkit::helpers::{Clip2x, biased};

pub struct Peace {
    hp: Biquad,
    clip: Clip2x,
    mid: Biquad,
    treb: Biquad,
}

impl Peace {
    pub const NEW: Self = Self { hp: Biquad::NEW, clip: Clip2x::NEW, mid: Biquad::NEW, treb: Biquad::NEW };

    pub fn init(&mut self) {
        self.hp.highpass(70.0, 0.707);
        self.mid.peak(800.0, 0.7, 2.0);
        self.treb.peak(3000.0, 0.8, 2.5);
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let x = self.hp.process(x);
        let y = self.clip.process(x * db(PEACE_DRIVE_DB), |u| biased(u, BIAS1));
        self.treb.process(self.mid.process(y))
    }
}
