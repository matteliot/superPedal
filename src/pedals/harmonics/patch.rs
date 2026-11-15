//! The whole patch: the guitar in four modes (1 regular, 2 harmonics only, 3
//! low harmonics, 4 high harmonics), found by pitch tracking and filtering
//! around multiples of the tracked fundamental, into a clean amp, a small
//! cab and room. A computer voice says which mode has just been switched
//! into, with the guitar ducked under it.

use super::modes::Modes;
use super::pitch::Pitch;
use super::presets::*;
use crate::pedalkit::abi::Color;
use crate::pedalkit::clean::CleanAmp;
use crate::pedalkit::dsp::{Biquad, db};
use crate::pedalkit::eq::EqChain;
use crate::pedalkit::helpers::{Smooth, limit};
use crate::pedalkit::pedal::{Param, Pedal};
use crate::pedalkit::pool::Pool;
use crate::pedalkit::reverb::Reverb;
use crate::pedalkit::voice::Announcer;

/// Knob indices.
const BLEND: usize = 0;
const VOICE: usize = 1;
const LEVEL: usize = 2;

pub struct Patch {
    knobs: [f32; 3],
    mode: usize,
    out: Smooth,
    voice_g: Smooth,
    blend: Smooth,
    hp_in: Biquad,
    pitch: Pitch,
    modes: Modes,
    clean: CleanAmp,
    clean_eq: EqChain<2>,
    cab: EqChain<4>,
    rev: Reverb,
    voice: Announcer,
}

impl Patch {
    fn retarget(&mut self) {
        // Square law: 0 at minimum, unity at 3 o'clock, +5 dB at maximum.
        let v = self.knobs[LEVEL] / 0.75;
        self.out.set(v * v * OUT_TRIM);
        let vo = self.knobs[VOICE];
        self.voice_g.set(vo * vo * VOICE_MAX);
        self.blend.set(self.knobs[BLEND]);
    }

    /// Switch mode and announce it.
    fn set_mode(&mut self, mode: usize) {
        self.mode = mode % 4;
        self.voice.say(self.mode);
    }

    /// The tracked pitch (Hz), for host-side analysis.
    #[cfg(not(target_os = "none"))]
    #[allow(dead_code)]
    pub fn pitch(&self) -> f32 {
        self.pitch.f_s
    }
}

impl Pedal for Patch {
    const PARAMS: [Param; 3] = [
        // Dry guitar mixed back under modes 2-4; CCW is harmonics only.
        Param { name: "Blend", default: 0.0 },
        // Announcement volume; CCW is silent.
        Param { name: "Voice", default: VOICE_DEFAULT },
        // Output; unity at 3 o'clock.
        Param { name: "Level", default: 0.75 },
    ];
    const POOL: usize =
        Pitch::POOL + 2048 + 2 * (1214 + 1293 + 1389 + 1475 + 1547 + 1622 + 1694 + 1759 + 605 + 480 + 371 + 245) + 12 * 25;

    const NEW: Self = Self {
        knobs: [Self::PARAMS[0].default, Self::PARAMS[1].default, Self::PARAMS[2].default],
        mode: 0,
        out: Smooth::new(0.0),
        voice_g: Smooth::new(0.0),
        blend: Smooth::new(0.0),
        hp_in: Biquad::NEW,
        pitch: Pitch::NEW,
        modes: Modes::NEW,
        clean: CleanAmp::NEW,
        clean_eq: EqChain::NEW,
        cab: EqChain::NEW,
        rev: Reverb::NEW,
        voice: Announcer::new(&PROMPTS),
    };

    fn init(&mut self, pool: &mut Pool) {
        self.hp_in.highpass(70.0, 0.707);
        self.pitch.init(pool);
        self.modes.init(self.pitch.f_s, self.mode);
        self.clean.init(&CLEAN);
        self.clean_eq.init(&CLEAN_EQ);
        self.cab.init(&CAB);
        self.rev.init(&REVERB, pool);
        self.voice.init(DUCK_S);
        for s in [&mut self.out, &mut self.voice_g, &mut self.blend] {
            s.time(0.02);
        }
        self.retarget();
        self.out.snap();
        self.voice_g.snap();
        self.blend.snap();
    }

    fn set_knob(&mut self, idx: usize, v: f32) {
        self.knobs[idx] = v;
        self.retarget();
    }

    /// Footswitch press: next mode (1 -> 2 -> 3 -> 4 -> 1), announced.
    fn press(&mut self) {
        self.set_mode(self.mode + 1);
    }

    /// Footswitch hold: back to mode 1, announced.
    fn hold(&mut self) {
        self.set_mode(0);
    }

    #[inline]
    fn tick(&mut self, x: f32, _: f32) -> (f32, f32) {
        let out = self.out.tick();
        let vg = self.voice_g.tick();
        let bl = self.blend.tick();
        let x = self.hp_in.process(x);
        let f = self.pitch.process(x);
        let mut y = self.modes.process(x, f, self.mode);
        // Blend the dry guitar back under the harmonics (not in mode 1).
        y += bl * (1.0 - self.modes.gains[0]) * x;
        let y = self.clean_eq.process(self.clean.process(y)) * db(CLEAN_MAKEUP_DB);
        let y = self.cab.process(y);
        let (wl, wr) = self.rev.process(y, ROOM);
        // The announcement goes on top, with the guitar ducked under it.
        let v = self.voice.speak() * vg;
        let duck = self.voice.duck_step();
        let g = out * (1.0 - DUCK * duck * (vg / VOICE_MAX).min(1.0));
        (limit((y + WET * wl) * g + v), limit((y + WET * wr) * g + v))
    }

    fn led(&self) -> Color {
        match self.mode {
            0 => Color::DimWhite,
            1 => Color::LightBlue,
            2 => Color::DarkCobalt,
            _ => Color::LightYellow,
        }
    }
}
