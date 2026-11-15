//! A tape slapback: one delay whose repeat path has the head's bandwidth, a
//! low-mid head bump, a little tape squash and the capstan's wow and
//! flutter, so every repeat comes back darker and slightly smeared. ECHO sets
//! level and loop gain (knob^2, so the bottom of the knob is a single slap);
//! the flavour sets the time.

use super::presets::*;
use crate::pedalkit::dsp::{Biquad, OnePole, OnePoleHp, tanh};
use crate::pedalkit::pool::{Line, Pool};

/// Sine-ish LFO, -1..+1, from a phase accumulator and a parabola.
struct Lfo {
    phase: f32,
    inc: f32,
}

impl Lfo {
    const NEW: Self = Self { phase: 0.0, inc: 0.0 };

    #[inline]
    fn process(&mut self) -> f32 {
        let t = self.phase * 2.0 - 1.0;
        let y = 4.0 * t * (1.0 - t.abs());
        self.phase += self.inc;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        y
    }
}

pub struct TapeEcho {
    line: Line,
    lpf: OnePole,
    hpf: OnePoleHp,
    bump: Biquad,
    wow: Lfo,
    flutter: Lfo,
    time: f32,
    wow_samples: f32,
    flutter_samples: f32,
    level: f32,
    feedback: f32,
}

impl TapeEcho {
    pub const NEW: Self = Self {
        line: Line::NEW,
        lpf: OnePole::NEW,
        hpf: OnePoleHp::NEW,
        bump: Biquad::NEW,
        wow: Lfo::NEW,
        flutter: Lfo::NEW,
        time: 0.0,
        wow_samples: 0.0,
        flutter_samples: 0.0,
        level: 0.0,
        feedback: 0.0,
    };

    pub fn init(&mut self, pool: &mut Pool) {
        self.lpf.set_at(ECHO_LPF_HZ, FS);
        self.hpf.set_at(ECHO_HPF_HZ, FS);
        self.bump.peak_at(ECHO_BUMP_HZ, 0.7, ECHO_BUMP_DB, FS);
        self.wow.inc = ECHO_WOW_HZ / FS;
        self.flutter.inc = ECHO_FLUTTER_HZ / FS;
        self.wow_samples = ECHO_WOW_MS * 0.001 * FS;
        self.flutter_samples = ECHO_FLUTTER_MS * 0.001 * FS;
        self.line.take(pool, ECHO_BUFFER);
        self.reset();
    }

    pub fn reset(&mut self) {
        self.line.clear();
        self.lpf.reset();
        self.hpf.reset();
        self.bump.reset();
        self.wow.phase = 0.0;
        self.flutter.phase = 0.0;
    }

    pub fn set_time(&mut self, ms: f32) {
        self.time = ms * 0.001 * FS;
    }

    pub fn set(&mut self, knob: f32) {
        self.level = knob * ECHO_LEVEL_MAX;
        self.feedback = knob * knob * ECHO_FEEDBACK_MAX;
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let pos = self.time + self.wow_samples * self.wow.process() + self.flutter_samples * self.flutter.process();
        // `read(d)` counts from the newest sample; the C++ line's d from one past it.
        let v = self.line.read(pos - 1.0);
        let repeat = kill_denormal(tanh(self.bump.process(self.hpf.process(self.lpf.process(v)))));
        self.line.push(x + self.feedback * repeat);
        x + self.level * repeat
    }
}
