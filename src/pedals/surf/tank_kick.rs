//! Footswitch tap: Dick Dale slamming the reverb unit. A short filtered noise
//! splash (the springs clattering) plus a low decaying thud, injected into
//! the springs at a level that saturates the driver. One-shot.

use super::presets::*;
use crate::pedalkit::dsp::{OnePole, exp_hq, sin};
use core::f32::consts::PI;

const SEED: u32 = 0x9E37_79B9;

pub struct TankKick {
    noise_env: f32,
    thud_env: f32,
    noise_decay: f32,
    thud_decay: f32,
    thud_phase: f32,
    thud_inc: f32,
    level: f32,
    rng: u32,
    noise_color: OnePole,
}

impl TankKick {
    pub const NEW: Self = Self {
        noise_env: 0.0,
        thud_env: 0.0,
        noise_decay: 0.0,
        thud_decay: 0.0,
        thud_phase: 0.0,
        thud_inc: 0.0,
        level: 1.0,
        rng: SEED,
        noise_color: OnePole::NEW,
    };

    pub fn init(&mut self) {
        self.noise_decay = exp_hq(-1.0 / (KICK_NOISE_MS * 0.001 * FS));
        self.thud_decay = exp_hq(-1.0 / (KICK_THUD_MS * 0.001 * FS));
        self.thud_inc = KICK_THUD_HZ / FS;
        self.noise_color.set_at(2500.0, FS);
        self.reset();
    }

    /// Reseeded, so a crash is repeatable.
    pub fn reset(&mut self) {
        self.noise_env = 0.0;
        self.thud_env = 0.0;
        self.thud_phase = 0.0;
        self.rng = SEED;
        self.noise_color.reset();
    }

    pub fn trigger(&mut self, drip: f32) {
        self.level = KICK_LEVEL * (0.5 + drip);
        self.noise_env = 1.0;
        self.thud_env = 1.0;
        self.thud_phase = 0.0;
    }

    #[inline]
    pub fn process(&mut self) -> f32 {
        if !(self.noise_env > 1e-4 || self.thud_env > 1e-4) {
            return 0.0;
        }
        self.rng = self.rng.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let noise = (self.rng >> 8) as f32 / 8_388_608.0 - 1.0;
        let out = self.level
            * (self.noise_color.process(noise) * self.noise_env + 0.8 * sin(2.0 * PI * self.thud_phase) * self.thud_env);
        self.noise_env *= self.noise_decay;
        self.thud_env *= self.thud_decay;
        self.thud_phase += self.thud_inc;
        if self.thud_phase >= 1.0 {
            self.thud_phase -= 1.0;
        }
        out
    }
}
