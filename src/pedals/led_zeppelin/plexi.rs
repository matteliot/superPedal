//! Non-master-volume Marshall Super Lead: a bright cap, two triode stages
//! (the first 2x oversampled) with coupling caps, the Marshall tone stack
//! (lean bass, scooped low mids, treble up), then the EL34s, where most of
//! the crunch lives, on a sagging supply, and presence.

use super::presets::*;
use crate::pedalkit::dsp::{Biquad, coef, tanh};
use crate::pedalkit::helpers::{Clip2x, biased};

pub struct Plexi {
    bright: Biquad,
    clip: Clip2x,
    cpl: [Biquad; 2],
    stack: [Biquad; 3],
    presence: Biquad,
    sag: f32,
    c_sag: f32,
}

impl Plexi {
    pub const NEW: Self = Self {
        bright: Biquad::NEW,
        clip: Clip2x::NEW,
        cpl: [Biquad::NEW; 2],
        stack: [Biquad::NEW; 3],
        presence: Biquad::NEW,
        sag: 0.0,
        c_sag: 1.0,
    };

    pub fn init(&mut self) {
        self.bright.peak(3000.0, 0.7, 2.5);
        self.cpl[0].highpass(25.0, 0.707);
        self.cpl[1].highpass(25.0, 0.707);
        self.stack[0].peak(100.0, 0.8, -2.0);
        self.stack[1].peak(650.0, 0.8, -3.5);
        self.stack[2].peak(2500.0, 0.9, 2.0);
        self.presence.peak(3500.0, 0.9, 3.0);
        self.c_sag = coef(0.05);
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let v = self.bright.process(x) * PRE1_GAIN;
        let a = self.clip.process(v, triode);
        let a = self.cpl[0].process(a);
        let b = self.cpl[1].process(triode(a * PRE2_GAIN));
        let mut t = b;
        for s in self.stack.iter_mut() {
            t = s.process(t);
        }
        // The supply sags under loud notes (only once the power amp is working hard), squeezing it: the "bloom". Quiet playing never sags, so a backed-off guitar stays clean.
        let p = t * POWER_GAIN;
        self.sag += ((p.abs() - SAG_KNEE).max(0.0) - self.sag) * self.c_sag;
        let y = tanh(p / (1.0 + SAG * self.sag));
        self.presence.process(y)
    }
}

/// A triode stage: soft and slightly asymmetric (bias shifted), DC removed.
#[inline]
fn triode(v: f32) -> f32 {
    biased(v, TRIODE_BIAS)
}
