//! Bigsby: a short delay line swept unevenly, so the pitch wavers flat and
//! back. The signal always goes through it (2 samples at rest), so switching
//! it on and off never combs.

use super::presets::*;
use crate::pedalkit::dsp::{SR, cos, sin};
use crate::pedalkit::helpers::{Phase, Smooth};
use crate::pedalkit::pool::{Line, Pool};
use core::f32::consts::TAU;

pub struct Bigsby {
    line: Line,
    amt: Smooth,
    ph: Phase,
    drift: Phase,
    sway: Phase,
}

impl Bigsby {
    pub const NEW: Self =
        Self { line: Line::NEW, amt: Smooth::new(0.0), ph: Phase::new(0.0), drift: Phase::new(0.0), sway: Phase::new(0.5) };

    pub fn init(&mut self, pool: &mut Pool) {
        self.line.take(pool, 128);
        self.amt.time(WOB_HAND_S);
    }

    /// The hand on (`true`) or off the bar.
    pub fn on(&mut self, on: bool) {
        self.amt.on(on);
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let amt = self.amt.tick();
        let d = self.drift.step(WOB_DRIFT_HZ / SR);
        let s = self.sway.step(WOB_SWAY_HZ / SR);
        let rate = WOB_RATE * (1.0 + WOB_DRIFT * sin(TAU * d));
        let p = self.ph.step(rate / SR);
        let depth = WOB_DEPTH * amt * (1.0 - WOB_SWAY + WOB_SWAY * sin(TAU * s));
        self.line.push(x);
        self.line.read(WOB_BASE + depth * 0.5 * (1.0 - cos(TAU * p)))
    }
}
