//! Downward expander with hysteresis and a smoothed gain ramp, so high-gain
//! hiss disappears between notes without chattering.

use super::presets::*;
use crate::pedalkit::dsp::db;
use crate::pedalkit::env::ExpEnvelope;

pub struct HissGate {
    env: ExpEnvelope,
    open_at: f32,
    close_at: f32,
    gain: f32,
    open: bool,
}

impl HissGate {
    pub const NEW: Self = Self { env: ExpEnvelope::NEW, open_at: 0.0, close_at: 0.0, gain: 1.0, open: true };

    pub fn init(&mut self) {
        self.env.set(1.0, 60.0, FS);
        self.open_at = db(GATE_THRESH_DB);
        self.close_at = db(GATE_THRESH_DB - GATE_HYST_DB);
        self.reset();
    }

    pub fn reset(&mut self) {
        self.env.reset();
        self.gain = 1.0;
        self.open = true;
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let level = self.env.process(x);
        if self.open {
            if level < self.close_at {
                self.open = false;
            }
        } else if level > self.open_at {
            self.open = true;
        }
        let target = if self.open { 1.0 } else { 0.0 };
        self.gain += (target - self.gain) * if target > self.gain { GATE_ATTACK } else { GATE_RELEASE };
        x * self.gain
    }
}
