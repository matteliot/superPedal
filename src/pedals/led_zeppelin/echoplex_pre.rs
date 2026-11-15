//! Echoplex EP-3 preamp: always in the chain. A small, slightly asymmetric
//! boost with a touch of sparkle up top; the coupling cap clears the bias DC.

use super::presets::{EP_BIAS, EP_GAIN};
use crate::pedalkit::dsp::Biquad;
use crate::pedalkit::helpers::biased;

pub struct EchoplexPre {
    lift: Biquad,
    hp: Biquad,
}

impl EchoplexPre {
    pub const NEW: Self = Self { lift: Biquad::NEW, hp: Biquad::NEW };

    pub fn init(&mut self) {
        self.lift.peak(5000.0, 0.7, 2.5);
        self.hp.highpass(25.0, 0.707);
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let y = self.lift.process(x) * EP_GAIN;
        self.hp.process(biased(y, EP_BIAS))
    }
}
