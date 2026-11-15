//! Jaguar/Jazzmaster in a box: a high shelf and a presence peak for the top
//! end, plus a transient shaper that pushes the pick attack (fast envelope
//! minus slow envelope, applied as a short gain bump). SPANK scales all
//! three; the flavour adds a little extra shelf.

use super::presets::*;
use crate::pedalkit::dsp::Biquad;
use crate::pedalkit::env::ExpEnvelope;

pub struct Spank {
    shelf: Biquad,
    peak: Biquad,
    fast: ExpEnvelope,
    slow: ExpEnvelope,
    transient: f32,
}

impl Spank {
    pub const NEW: Self =
        Self { shelf: Biquad::NEW, peak: Biquad::NEW, fast: ExpEnvelope::NEW, slow: ExpEnvelope::NEW, transient: 0.0 };

    pub fn init(&mut self) {
        self.fast.set(SPANK_FAST_MS, SPANK_FAST_REL_MS, FS);
        self.slow.set(SPANK_SLOW_MS, SPANK_SLOW_REL_MS, FS);
        self.set(0.5, 0.0);
        self.reset();
    }

    pub fn reset(&mut self) {
        self.shelf.reset();
        self.peak.reset();
        self.fast.reset();
        self.slow.reset();
    }

    pub fn set(&mut self, knob: f32, extra_db: f32) {
        self.shelf.high_shelf_at(SPANK_SHELF_HZ, knob * SPANK_SHELF_DB + extra_db, FS);
        self.peak.peak_at(SPANK_PEAK_HZ, 1.0, knob * SPANK_PEAK_DB, FS);
        self.transient = knob * SPANK_TRANSIENT;
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let y = self.peak.process(self.shelf.process(x));
        // >1 only while the fast envelope is ahead of the slow one, i.e.
        // during the first milliseconds of a pick.
        let f = self.fast.process(x);
        let s = self.slow.process(x);
        let attack = ((f - s) / (s + 0.01)).clamp(0.0, 1.0);
        y * (1.0 + self.transient * attack)
    }
}
