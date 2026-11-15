//! Whammy, octave up, before the amp: each pick attack drops the pedal to the
//! heel and it rocks straight back up an octave, so every note swoops up into
//! the octave and stays there until the next one. A pitch shifter of two
//! crossfaded taps sweeping through a short delay line.

use super::presets::*;
use crate::pedalkit::dsp::{SR, coef, cos, exp2, floor};
use crate::pedalkit::helpers::Smooth;
use crate::pedalkit::pool::{Line, Pool};
use core::f32::consts::TAU;

pub struct Whammy {
    line: Line,
    ph: f32,
    semi: f32,
    mix: Smooth,
    lock: u32,
    c_rise: f32,
}

impl Whammy {
    pub const NEW: Self = Self { line: Line::NEW, ph: 0.0, semi: WH_SEMIS, mix: Smooth::new(0.0), lock: 0, c_rise: 1.0 };

    pub fn init(&mut self, pool: &mut Pool) {
        self.line.take(pool, 4096);
        self.c_rise = coef(WH_RISE_S);
        self.mix.time(WH_FADE_S);
    }

    /// `env`/`slow` the fast and slow input envelopes (for the pick
    /// detector); `armed` fades the octave in.
    #[inline]
    pub fn process(&mut self, x: f32, env: f32, slow: f32, armed: bool) -> f32 {
        if self.lock > 0 {
            self.lock -= 1;
        } else if env > WH_ONSET * slow && env > 2.0 * GATE_OPEN {
            self.semi = 0.0;
            self.lock = (WH_LOCK_S * SR) as u32;
        }
        self.semi += (WH_SEMIS - self.semi) * self.c_rise;
        self.mix.on(armed);
        let m = self.mix.tick();
        let s = self.shift(x, exp2(self.semi * (1.0 / 12.0)));
        x + (s - x) * m
    }

    /// The read point moves at `ratio` times the write point.
    #[inline]
    fn shift(&mut self, x: f32, ratio: f32) -> f32 {
        self.line.push(x);
        self.ph += (ratio - 1.0) * (1.0 / WH_WIN);
        self.ph -= floor(self.ph);
        let mut p2 = self.ph + 0.5;
        if p2 >= 1.0 {
            p2 -= 1.0;
        }
        // Hann windows half a window apart sum to 1; each is silent when its
        // tap jumps back.
        let g1 = 0.5 - 0.5 * cos(TAU * self.ph);
        let a = self.line.read((1.0 - self.ph) * WH_WIN + 1.0);
        let c = self.line.read((1.0 - p2) * WH_WIN + 1.0);
        a * g1 + c * (1.0 - g1)
    }
}
