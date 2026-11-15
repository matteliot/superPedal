//! One spring: a delay loop whose feedback path runs through a dispersion
//! chain (second-order allpasses: the textbook model of a spring's
//! dispersion, each echo smeared into an upward chirp, the "boing" heard as
//! drip), a damping low-pass (tail brightness) and a high-pass (springs carry
//! no bass). The loop gain sets the decay; the output is taken after the
//! dispersion, so every echo, including the first, carries the chirp.

use super::presets::*;
use crate::pedalkit::dsp::{OnePole, OnePoleHp, cos};
use crate::pedalkit::pool::{Line, Pool};
use core::f32::consts::PI;

/// Second-order allpass, pole pair at radius r and angle 2 pi fc / fs:
/// unity gain everywhere, but the group delay humps at fc.
#[derive(Clone, Copy)]
struct Allpass2 {
    c1: f32,
    c2: f32,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl Allpass2 {
    const NEW: Self = Self { c1: 0.0, c2: 0.0, x1: 0.0, x2: 0.0, y1: 0.0, y2: 0.0 };

    fn set(&mut self, fc: f32, r: f32) {
        self.c1 = -2.0 * r * cos(2.0 * PI * fc / FS);
        self.c2 = r * r;
    }

    fn reset(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }

    #[inline]
    fn process(&mut self, x: f32) -> f32 {
        let y = self.c2 * x + self.c1 * self.x1 + self.x2 - self.c1 * self.y1 - self.c2 * self.y2;
        self.x2 = self.x1;
        self.x1 = x;
        self.y2 = self.y1;
        self.y1 = y;
        y
    }
}

pub struct Spring {
    line: Line,
    disperse: [Allpass2; ALLPASS_STAGES],
    damp: OnePole,
    hpf: OnePoleHp,
    length: f32,
    feedback: f32,
}

impl Spring {
    pub const NEW: Self = Self {
        line: Line::NEW,
        disperse: [Allpass2::NEW; ALLPASS_STAGES],
        damp: OnePole::NEW,
        hpf: OnePoleHp::NEW,
        length: 0.0,
        feedback: 0.0,
    };

    pub fn init(&mut self, ms: f32, chirp_hz: f32, pool: &mut Pool) {
        self.length = ms * 0.001 * FS;
        for s in self.disperse.iter_mut() {
            s.set(chirp_hz, SPRING_CHIRP_R);
        }
        self.hpf.set_at(SPRING_HPF_HZ, FS);
        self.line.take(pool, SPRING_BUFFER);
        self.reset();
    }

    pub fn set_decay(&mut self, loop_gain: f32, damp_hz: f32) {
        self.feedback = loop_gain;
        self.damp.set_at(damp_hz, FS);
    }

    pub fn reset(&mut self) {
        self.line.clear();
        for s in self.disperse.iter_mut() {
            s.reset();
        }
        self.damp.reset();
        self.hpf.reset();
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        // `read(d)` counts from the newest sample; the C++ line's d from one past it.
        let mut echo = self.line.read(self.length - 1.0);
        for s in self.disperse.iter_mut() {
            echo = s.process(echo);
        }
        let last = kill_denormal(echo);
        self.line.push(x + self.feedback * self.hpf.process(self.damp.process(last)));
        last
    }
}
