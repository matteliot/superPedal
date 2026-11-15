//! Feedback as a sustainer (Downtown only): once a note is left ringing, the
//! amp takes it over, locking onto a harmonic and wandering between them
//! (see FB_WAIT in `presets`). Picking again, or muting, kills it.

use super::presets::*;
use crate::pedalkit::dsp::{Biquad, SR, coef, db, exp2, sin};
use crate::pedalkit::env::{Follower, Onset};
use crate::pedalkit::helpers::Phase;
use core::f32::consts::TAU;

pub struct Sustainer {
    env: Follower,
    onset: Onset,
    pub since_onset: u32,
    on: bool,
    reference: f32,
    pub ramp: f32,
    ph: Phase,
    count: u32,
    bp: Biquad,
    c_up: f32,
    c_down: f32,
}

impl Sustainer {
    pub const NEW: Self = Self {
        env: Follower::NEW,
        onset: Onset::NEW,
        since_onset: 0,
        on: false,
        reference: 0.0,
        ramp: 0.0,
        ph: Phase::new(0.0),
        count: 0,
        bp: Biquad::NEW,
        c_up: 1.0,
        c_down: 1.0,
    };

    pub fn init(&mut self) {
        self.env.times(0.001, 0.05);
        self.onset.time(0.03);
        self.bp.bandpass(FB_FREQ, FB_Q);
        self.c_up = coef(FB_BLOOM_S);
        self.c_down = coef(FB_DIE_S);
    }

    /// Follows the guitar (before the Bigsby) and spots pick attacks.
    #[inline]
    pub fn track(&mut self, x: f32) {
        let env = self.env.follow(x.abs());
        self.onset.update(env);
        if self.onset.hit(env, ONSET, ONSET_MIN) {
            self.since_onset = 0;
        } else {
            self.since_onset = self.since_onset.saturating_add(1);
        }
    }

    /// The guitar as the amp hears it: `x` itself, or the sustained note.
    #[inline]
    pub fn process(&mut self, x: f32, loud: bool) -> f32 {
        let p = self.ph.step(FB_RATE / SR);
        self.count = self.count.wrapping_add(1);
        if self.count & 31 == 0 {
            self.bp.bandpass(FB_FREQ * exp2(FB_WANDER * sin(TAU * p)), FB_Q);
        }
        let h = self.bp.process(x);
        let env = self.env.env;
        if self.since_onset == 0 || env < FB_MUTE || !loud {
            self.on = false;
        } else if !self.on && self.since_onset as f32 > FB_WAIT * SR && env > FB_HOLD {
            self.on = true;
            self.reference = env;
        }
        let ft = if self.on { 1.0 } else { 0.0 };
        self.ramp += (ft - self.ramp) * if ft > self.ramp { self.c_up } else { self.c_down };
        let hold = (self.reference / env.max(1e-6)).min(db(FB_MAX_DB));
        x + (hold * (FB_DRY * x + FB_HARM * h) - x) * self.ramp
    }
}
