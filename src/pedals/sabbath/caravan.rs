//! Planet Caravan: one gentle tube stage into the Leslie.

use super::leslie::Leslie;
use super::presets::*;
use super::tube_stage::TubeStage;
use crate::pedalkit::dsp::{Biquad, powf};
use crate::pedalkit::pool::Pool;

pub struct Caravan {
    input_hpf: Biquad,
    warmth: TubeStage,
    pub leslie: Leslie,
}

impl Caravan {
    pub const NEW: Self = Self { input_hpf: Biquad::NEW, warmth: TubeStage::NEW, leslie: Leslie::NEW };

    pub fn init(&mut self, pool: &mut Pool) {
        self.input_hpf.highpass_at(INPUT_HPF_HZ, 0.707, FS);
        self.warmth.init(PC_GAIN, PC_BIAS, PC_COUPLING_HZ, FS);
        self.leslie.init(pool);
        self.reset();
    }

    pub fn reset(&mut self) {
        self.input_hpf.reset();
        self.warmth.reset();
        self.leslie.reset();
    }

    /// GAIN knob -> rotor depth.
    pub fn set_depth(&mut self, knob: f32) {
        self.leslie.set_depth(knob);
    }

    /// TONE knob -> rotor rate, exponential from chorale to tremolo.
    pub fn set_rate(&mut self, knob: f32) {
        self.leslie.set_rate(PC_RATE_MIN_HZ * powf(PC_RATE_MAX_HZ / PC_RATE_MIN_HZ, knob));
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> (f32, f32) {
        self.leslie.process(self.warmth.process(self.input_hpf.process(x)))
    }
}
