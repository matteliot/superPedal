//! Psychocandy fuzz: thin and trebly in, a Super-Fuzz-style first stage
//! (saturation plus its full-wave rectified copy, an octave up) slammed into
//! a hard clipper, then a scooped body with a fierce upper-mid bite.

use super::presets::{FUZZ2_DB, OCTAVE};
use crate::pedalkit::dsp::{Biquad, OnePole, db, tanh};
use crate::pedalkit::helpers::{Clip2x, hard};

pub struct Fuzz {
    hp: Biquad,
    emph: OnePole,
    clip1: Clip2x,
    inter_hp: Biquad,
    inter_lp: Biquad,
    clip2: Clip2x,
    scoop: Biquad,
    bite: Biquad,
    lp: Biquad,
}

impl Fuzz {
    pub const NEW: Self = Self {
        hp: Biquad::NEW,
        emph: OnePole::NEW,
        clip1: Clip2x::NEW,
        inter_hp: Biquad::NEW,
        inter_lp: Biquad::NEW,
        clip2: Clip2x::NEW,
        scoop: Biquad::NEW,
        bite: Biquad::NEW,
        lp: Biquad::NEW,
    };

    pub fn init(&mut self) {
        // Nothing below ~140 Hz into the fuzz.
        self.hp.highpass(140.0, 0.707);
        self.emph.set(900.0);
        self.inter_hp.highpass(160.0, 0.707); // also strips the octave's DC
        self.inter_lp.lowpass(6500.0, 0.707);
        // Scooped body, a fierce upper-mid bite, fizz left on top.
        self.scoop.peak(650.0, 0.7, -5.0);
        self.bite.peak(2800.0, 1.0, 6.0);
        self.lp.lowpass(8500.0, 0.707);
    }

    /// `g` the first stage's gain.
    #[inline]
    pub fn process(&mut self, x: f32, g: f32) -> f32 {
        let x = self.hp.process(x);
        let x = x + 2.0 * (x - self.emph.process(x)); // ~+10 dB above 900 Hz
        let y = self.clip1.process(x * g, octave);
        let y = self.inter_lp.process(self.inter_hp.process(y));
        let y = self.clip2.process(y * db(FUZZ2_DB), hard);
        let y = self.bite.process(self.scoop.process(y));
        self.lp.process(y)
    }
}

/// Fuzz stage 1: saturation plus its full-wave rectified copy (octave up).
#[inline]
fn octave(u: f32) -> f32 {
    let a = tanh(u);
    a + OCTAVE * a.abs()
}
