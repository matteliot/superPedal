//! 2x oversampler: a linear-interpolated midpoint plus an image filter on
//! the way up, an anti-alias filter and decimation on the way down. The
//! nonlinear chain runs twice per input sample.

use super::presets::{FS_OS, OS_FILTER_HZ};
use crate::pedalkit::dsp::Biquad;

pub struct Oversampler {
    up: Biquad,
    down: Biquad,
    prev: f32,
}

impl Oversampler {
    pub const NEW: Self = Self { up: Biquad::NEW, down: Biquad::NEW, prev: 0.0 };

    pub fn init(&mut self) {
        self.up.lowpass_at(OS_FILTER_HZ, 0.7, FS_OS);
        self.down.lowpass_at(OS_FILTER_HZ, 0.7, FS_OS);
        self.reset();
    }

    pub fn reset(&mut self) {
        self.up.reset();
        self.down.reset();
        self.prev = 0.0;
    }

    #[inline]
    pub fn process(&mut self, x: f32, mut nonlinear: impl FnMut(f32) -> f32) -> f32 {
        let mid = self.up.process((self.prev + x) * 0.5);
        let cur = self.up.process(x);
        self.prev = x;
        // Computed only to keep the filter's state right; decimation drops it.
        self.down.process(nonlinear(mid));
        self.down.process(nonlinear(cur))
    }
}
