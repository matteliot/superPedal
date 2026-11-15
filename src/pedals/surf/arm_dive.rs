//! A Jazzmaster vibrato arm, pushed and let go. Pitch is shifted by moving
//! the read point of a delay line: while the pitch is below nominal the delay
//! grows, while it is above it shrinks. The push is a raised-cosine drop to
//! -DIVE_SEMIS, the release a damped wobble around nominal (the arm's spring
//! overshoots). That leaves the read point ~15 ms behind, so once the wobble
//! has died the output is crossfaded back to the base delay.

use super::presets::*;
use crate::pedalkit::dsp::{cos, exp_hq, sin};
use crate::pedalkit::pool::{Line, Pool};
use core::f32::consts::PI;

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    Idle,
    Down,
    Wobble,
    Return,
}

pub struct ArmDive {
    line: Line,
    phase: Phase,
    delay: f32,
    return_from: f32,
    t: f32,
    down_samples: f32,
    wobble_samples: f32,
    return_samples: f32,
    wobble_decay: f32,
    wobble_inc: f32,
    max_delay: f32,
    wobble_phase: f32,
    wobble_env: f32,
}

impl ArmDive {
    pub const NEW: Self = Self {
        line: Line::NEW,
        phase: Phase::Idle,
        delay: DIVE_BASE_SAMPLES,
        return_from: 0.0,
        t: 0.0,
        down_samples: 0.0,
        wobble_samples: 0.0,
        return_samples: 0.0,
        wobble_decay: 0.0,
        wobble_inc: 0.0,
        max_delay: 0.0,
        wobble_phase: 0.0,
        wobble_env: 0.0,
    };

    pub fn init(&mut self, pool: &mut Pool) {
        self.down_samples = DIVE_DOWN_MS * 0.001 * FS;
        self.wobble_samples = DIVE_WOBBLE_MS * 0.001 * FS;
        self.return_samples = DIVE_RETURN_MS * 0.001 * FS;
        self.wobble_decay = exp_hq(-1.0 / (DIVE_WOBBLE_TAU_MS * 0.001 * FS));
        self.wobble_inc = DIVE_WOBBLE_HZ / FS;
        self.line.take(pool, DIVE_BUFFER);
        self.max_delay = (DIVE_BUFFER - 3) as f32;
        self.reset();
    }

    pub fn reset(&mut self) {
        self.line.clear();
        self.phase = Phase::Idle;
        self.delay = DIVE_BASE_SAMPLES;
        self.t = 0.0;
    }

    pub fn trigger(&mut self) {
        self.phase = Phase::Down;
        self.t = 0.0;
    }

    pub fn active(&self) -> bool {
        self.phase != Phase::Idle
    }

    /// `read(d)` counts from the newest sample; the C++ line's d from one past it.
    #[inline]
    fn read(&self, d: f32) -> f32 {
        self.line.read(d - 1.0)
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        self.line.push(x);
        let y = if self.phase == Phase::Return { self.return_mix() } else { self.read(self.delay) };
        self.advance();
        y
    }

    /// Semitones away from nominal right now: the push, then the spring-back.
    fn semitones(&self) -> f32 {
        match self.phase {
            Phase::Down => -DIVE_SEMIS * 0.5 * (1.0 - cos(PI * self.t / self.down_samples)),
            Phase::Wobble => -DIVE_SEMIS * self.wobble_env * cos(2.0 * PI * self.wobble_phase),
            _ => 0.0,
        }
    }

    /// Pitch ratio for small shifts: 2^(s/12) ~ 1 + 0.0578 s (within 1% over +-3 semitones).
    fn ratio(semis: f32) -> f32 {
        1.0 + 0.05776 * semis
    }

    fn advance(&mut self) {
        if self.phase == Phase::Idle {
            return;
        }
        self.delay = (self.delay + (1.0 - Self::ratio(self.semitones()))).clamp(DIVE_BASE_SAMPLES, self.max_delay);
        self.t += 1.0;
        if self.phase == Phase::Down && self.t >= self.down_samples {
            self.phase = Phase::Wobble;
            self.t = 0.0;
            self.wobble_env = 1.0;
            self.wobble_phase = 0.0;
        } else if self.phase == Phase::Wobble {
            self.wobble_env *= self.wobble_decay;
            self.wobble_phase += self.wobble_inc;
            if self.wobble_phase >= 1.0 {
                self.wobble_phase -= 1.0;
            }
            if self.t >= self.wobble_samples {
                self.phase = Phase::Return;
                self.t = 0.0;
                self.return_from = self.delay;
            }
        } else if self.phase == Phase::Return && self.t >= self.return_samples {
            self.phase = Phase::Idle;
            self.delay = DIVE_BASE_SAMPLES;
        }
    }

    /// Equal-power crossfade from the late read point to the base one; both
    /// are read from the same line, so nothing is lost, only doubled for the
    /// fade's length.
    fn return_mix(&self) -> f32 {
        let f = self.t / self.return_samples;
        self.read(self.return_from) * cos(f * PI * 0.5) + self.read(DIVE_BASE_SAMPLES) * sin(f * PI * 0.5)
    }
}
