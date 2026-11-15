//! Leslie rotary: an 800 Hz crossover into a fast horn and a slower drum,
//! each a Doppler delay plus amplitude modulation, panned opposite ways for
//! stereo.

use super::presets::*;
use crate::pedalkit::dsp::{Biquad, cos, floor, sin};
use crate::pedalkit::pool::{Line, Pool};
use core::f32::consts::TAU;

/// Delay line length: more than (4 + 2.5) ms at 48 kHz.
pub const DELAY_SIZE: usize = 1024;

pub struct Leslie {
    low_split: Biquad,
    high_split: Biquad,
    horn: Line,
    drum: Line,
    /// The horn's rotor phase, 0..1 (the LED blinks with it).
    pub phase: f32,
    inc: f32,
    delay_centre: f32,
    delay_swing: f32,
    am_depth: f32,
}

impl Leslie {
    pub const NEW: Self = Self {
        low_split: Biquad::NEW,
        high_split: Biquad::NEW,
        horn: Line::NEW,
        drum: Line::NEW,
        phase: 0.0,
        inc: 0.0,
        delay_centre: 0.0,
        delay_swing: 0.0,
        am_depth: 0.0,
    };

    pub fn init(&mut self, pool: &mut Pool) {
        self.low_split.lowpass_at(PC_CROSSOVER_HZ, 0.707, FS);
        self.high_split.highpass_at(PC_CROSSOVER_HZ, 0.707, FS);
        self.delay_centre = PC_DELAY_MS * 0.001 * FS;
        self.set_depth(0.5);
        self.horn.take(pool, DELAY_SIZE);
        self.drum.take(pool, DELAY_SIZE);
        self.reset();
    }

    pub fn reset(&mut self) {
        self.low_split.reset();
        self.high_split.reset();
        self.horn.clear();
        self.drum.clear();
        self.phase = 0.0;
    }

    pub fn set_rate(&mut self, hz: f32) {
        self.inc = hz / FS;
    }

    pub fn set_depth(&mut self, d: f32) {
        self.delay_swing = PC_DEPTH_MAX_MS * 0.001 * FS * d;
        self.am_depth = PC_AM_DEPTH_MAX * d;
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> (f32, f32) {
        self.phase += self.inc;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        self.horn.push(self.high_split.process(x));
        self.drum.push(self.low_split.process(x));
        let p = self.phase * PC_DRUM_RATIO + PC_DRUM_OFFSET;
        let drum_phase = p - floor(p);
        let (hl, hr) = self.rotor(&self.horn, self.phase, 1.0);
        let (dl, dr) = self.rotor(&self.drum, drum_phase, 0.5);
        ((hl + dl) * PC_OUT_TRIM, (hr + dr) * PC_OUT_TRIM)
    }

    /// One rotor's delay line at its current angle: the delay swings with
    /// the sine of the angle (Doppler), the amplitude with the cosine (facing
    /// toward / away), mirrored for the other ear.
    #[inline]
    fn rotor(&self, line: &Line, angle: f32, scale: f32) -> (f32, f32) {
        let (s, c) = (sin(TAU * angle), cos(TAU * angle));
        let swing = self.delay_swing * scale;
        let am = self.am_depth * if scale < 1.0 { 0.6 } else { 1.0 };
        // `read(d)` is d samples behind the newest; the C++ line's d counts from one past it.
        let l = line.read(self.delay_centre + swing * s - 1.0) * (1.0 + am * c);
        let r = line.read(self.delay_centre - swing * s - 1.0) * (1.0 - am * c);
        (l, r)
    }
}
