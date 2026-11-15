//! A live room heard from distant mics: early reflections off a pre-delay
//! line, alternating sides, then a short Freeverb-style tail.

use super::presets::{ER_TAPS, ROOM_FEEDBACK, ROOM_PREDELAY_MS, ROOM_TAIL};
use crate::pedalkit::dsp::SR;
use crate::pedalkit::pool::{Line, Pool};
use crate::pedalkit::reverb::Reverb;

pub struct Room {
    pre: Line,
    tail: Reverb,
}

impl Room {
    pub const NEW: Self = Self { pre: Line::NEW, tail: Reverb::NEW };

    pub fn init(&mut self, pool: &mut Pool) {
        self.pre.take(pool, 4096);
        self.tail.init(&ROOM_TAIL, pool);
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> (f32, f32) {
        self.pre.push(x);
        // Early reflections alternate sides, each also heard (softer) on the other.
        let (mut el, mut er) = (0.0, 0.0);
        for (i, (ms, g)) in ER_TAPS.iter().enumerate() {
            let s = self.pre.read(ms * (SR / 1000.0)) * g;
            if i % 2 == 0 {
                el += s;
                er += 0.5 * s;
            } else {
                er += s;
                el += 0.5 * s;
            }
        }
        let (yl, yr) = self.tail.process(self.pre.read(ROOM_PREDELAY_MS * (SR / 1000.0)), ROOM_FEEDBACK);
        (0.5 * el + yl, 0.5 * er + yr)
    }
}
