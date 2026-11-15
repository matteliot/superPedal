//! Low shelf + mid peak + high shelf, morphed by one knob between a dark and
//! a bright voicing. Runs in the oversampled domain, after the last gain
//! stage, where it lives on the real amp.

use super::presets::*;
use crate::pedalkit::dsp::Biquad;

pub struct ToneStack {
    low: Biquad,
    mid: Biquad,
    high: Biquad,
}

impl ToneStack {
    pub const NEW: Self = Self { low: Biquad::NEW, mid: Biquad::NEW, high: Biquad::NEW };

    pub fn init(&mut self) {
        self.set(0.5);
        self.reset();
    }

    pub fn reset(&mut self) {
        self.low.reset();
        self.mid.reset();
        self.high.reset();
    }

    /// `t` 0..1: dark/sludge to bright/cutting.
    pub fn set(&mut self, t: f32) {
        self.low.low_shelf_at(LOW_SHELF_HZ, lerp(LOW_DB_DARK, LOW_DB_BRIGHT, t), FS_OS);
        self.mid.peak_at(MID_HZ, 0.7, lerp(MID_DB_DARK, MID_DB_BRIGHT, t), FS_OS);
        self.high.high_shelf_at(HIGH_SHELF_HZ, lerp(HIGH_DB_DARK, HIGH_DB_BRIGHT, t), FS_OS);
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        self.high.process(self.mid.process(self.low.process(x)))
    }
}

#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
