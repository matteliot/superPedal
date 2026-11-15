//! Echoplex tape echo: one repeat per pass, each a little darker, thinner and
//! more saturated, with wow and flutter.

use super::presets::*;
use crate::pedalkit::dsp::{Biquad, SR, sin, tanh};
use crate::pedalkit::helpers::Phase;
use crate::pedalkit::pool::{Line, Pool};
use core::f32::consts::TAU;

pub struct TapeEcho {
    tape: Line,
    hp: Biquad,
    lp: Biquad,
    wow: Phase,
    flutter: Phase,
}

impl TapeEcho {
    pub const NEW: Self = Self { tape: Line::NEW, hp: Biquad::NEW, lp: Biquad::NEW, wow: Phase::new(0.0), flutter: Phase::new(0.0) };

    pub fn init(&mut self, pool: &mut Pool) {
        self.tape.take(pool, 32768);
        self.hp.highpass(150.0, 0.707);
        self.lp.lowpass(3200.0, 0.707);
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let w = self.wow.step(WOW_HZ / SR);
        let f = self.flutter.step(FLUTTER_HZ / SR);
        let d = ECHO_S * SR * (1.0 + WOW_DEPTH * sin(TAU * w) + FLUTTER_DEPTH * sin(TAU * f));
        let y = self.tape.read(d);
        let fb = self.lp.process(self.hp.process(y));
        self.tape.push(tanh(x + fb * ECHO_FB));
        y
    }
}
