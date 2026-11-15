//! The clean's space: a stereo chorus (two taps swept in quadrature) and a
//! dotted-eighth echo, both scaled by the Space knob.

use super::presets::*;
use crate::pedalkit::dsp::{OnePole, SR, cos, sin};
use crate::pedalkit::helpers::Phase;
use crate::pedalkit::pool::{Line, Pool};
use core::f32::consts::TAU;

pub struct Space {
    cho: Line,
    ph: Phase,
    echo: Line,
    echo_lp: OnePole,
}

impl Space {
    pub const NEW: Self = Self { cho: Line::NEW, ph: Phase::new(0.0), echo: Line::NEW, echo_lp: OnePole::NEW };

    pub fn init(&mut self, pool: &mut Pool) {
        self.cho.take(pool, 1024);
        self.echo.take(pool, 32768);
        self.echo_lp.set(ECHO_LP);
    }

    /// The chorus at Space `sp`, on the clean only (`m` is the distortion's mix).
    #[inline]
    pub fn chorus(&mut self, y: f32, sp: f32, m: f32) -> (f32, f32) {
        self.cho.push(y);
        let w = TAU * self.ph.step(CHO_RATE / SR);
        let cho = CHO_MIX * sp * (1.0 - m);
        (y + cho * self.cho.read(CHO_DELAY + CHO_DEPTH * sin(w)), y + cho * self.cho.read(CHO_DELAY + CHO_DEPTH * cos(w)))
    }

    /// The echo's return; `send` goes in.
    #[inline]
    pub fn echo(&mut self, send: f32) -> f32 {
        let e = self.echo.read(ECHO_TIME);
        let fb = self.echo_lp.process(e) * ECHO_FB;
        self.echo.push(send + fb);
        e
    }
}
