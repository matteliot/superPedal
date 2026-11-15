//! Leslie rotary: an 800 Hz crossover into a horn and a drum turning in
//! opposite directions, each heard by two mics as a Doppler delay whose
//! loudness follows how directly the rotor faces the mic.

use super::presets::*;
use crate::pedalkit::dsp::{OnePole, SR, coef, cos, sin};
use crate::pedalkit::pool::{Line, Pool};
use core::f32::consts::TAU;

pub struct Rotary {
    xover: OnePole,
    horn: Line,
    drum: Line,
    horn_ph: f32,
    drum_ph: f32,
    horn_hz: f32,
    drum_hz: f32,
    mic_r: (f32, f32),
    c_horn: f32,
    c_drum: f32,
}

impl Rotary {
    pub const NEW: Self = Self {
        xover: OnePole::NEW,
        horn: Line::NEW,
        drum: Line::NEW,
        horn_ph: 0.0,
        drum_ph: 0.0,
        horn_hz: HORN_SLOW,
        drum_hz: DRUM_SLOW,
        mic_r: (1.0, 0.0),
        c_horn: 1.0,
        c_drum: 1.0,
    };

    pub fn init(&mut self, pool: &mut Pool) {
        self.horn.take(pool, 128);
        self.drum.take(pool, 64);
        self.xover.set(XOVER_HZ);
        self.mic_r = (cos(MIC_R_ANGLE), sin(MIC_R_ANGLE));
        self.c_horn = coef(HORN_INERTIA_S);
        self.c_drum = coef(DRUM_INERTIA_S);
    }

    #[inline]
    pub fn process(&mut self, y: f32, fast: bool) -> (f32, f32) {
        let (ht, dt) = if fast { (HORN_FAST, DRUM_FAST) } else { (HORN_SLOW, DRUM_SLOW) };
        self.horn_hz += (ht - self.horn_hz) * self.c_horn;
        self.drum_hz += (dt - self.drum_hz) * self.c_drum;
        // Horn and drum turn in opposite directions.
        self.horn_ph += TAU * self.horn_hz / SR;
        if self.horn_ph > TAU {
            self.horn_ph -= TAU;
        }
        self.drum_ph -= TAU * self.drum_hz / SR;
        if self.drum_ph < 0.0 {
            self.drum_ph += TAU;
        }
        let lo = self.xover.process(y);
        self.horn.push(y - lo);
        self.drum.push(lo);
        let (hc, hs) = (cos(self.horn_ph), sin(self.horn_ph));
        let (dc, ds) = (cos(self.drum_ph), sin(self.drum_ph));
        let (mc, ms) = self.mic_r;
        // facing = cos(rotor - mic) in [-1, 1]: 1 when the rotor points at the mic.
        let l = tap(&self.horn, hc, HORN_DOPPLER, HORN_AM) + tap(&self.drum, dc, DRUM_DOPPLER, DRUM_AM);
        let r = tap(&self.horn, hc * mc + hs * ms, HORN_DOPPLER, HORN_AM)
            + tap(&self.drum, dc * mc + ds * ms, DRUM_DOPPLER, DRUM_AM);
        (l, r)
    }
}

/// One rotor tap: Doppler delay and loudness follow how directly it faces the mic.
#[inline]
fn tap(d: &Line, facing: f32, doppler: f32, am: f32) -> f32 {
    let away = (1.0 - facing) * 0.5; // 0 facing the mic .. 1 facing away
    d.read(ROT_BASE + doppler * away) * (1.0 - am * away)
}
