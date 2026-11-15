//! superPedal: every pedal in the rack in one patch.
//!
//!   Left knob   picks the pedal: its travel is split into equal zones, "no
//!               pedal" first, then each pedal in alphabetical order, and the
//!               pedal's name is spoken as the knob enters its zone
//!   Mid knob    the selected pedal's own left knob (Gain, Fuzz, Drive, ...)
//!   Right knob  the selected pedal's own mid knob (Tone, Room, Space, ...)
//!   Footswitch  the selected pedal's own press and hold
//!   LED         the selected pedal's own (dim white for no pedal)
//! Each pedal's own right knob (mostly Level) stays at its default, and its
//! output is trimmed so every pedal is equally loud: the amp sets the volume.

use crate::pedalkit::abi::Color;
use crate::pedalkit::dsp::db;
use crate::pedalkit::helpers::{Smooth, soft_limit};
use crate::pedalkit::pedal::{Param, Pedal};
use crate::pedalkit::pool::Pool;
use crate::pedalkit::voice::Announcer;
use crate::presets::*;
use crate::pedalkit::abi::WORKING_BUFFER_SIZE;
use crate::rack::{MAX_POOL, POSITIONS, PROMPTS, Rack};
use crate::selector::Selector;

// Every pedal's delay memory fits the firmware's working buffer.
const _: () = assert!(MAX_POOL <= WORKING_BUFFER_SIZE as usize);

/// Knob indices.
const PEDAL: usize = 0;
const CTRL_1: usize = 1;
const CTRL_2: usize = 2;

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    /// The active pedal plays.
    Playing,
    /// Fading the active pedal out, to change to the target.
    Fading,
    /// The new pedal's memory is being cleared from this float on (silent).
    Clearing(usize),
}

pub struct SuperPedal {
    knobs: [f32; 3],
    selector: Selector,
    /// The position playing, and the one the knob points at.
    active: usize,
    target: usize,
    phase: Phase,
    fade: Smooth,
    trim: f32,
    rack: Rack,
    pool: Pool,
    has_pool: bool,
    voice: Announcer,
    /// The first audio block has run: knob moves from here on are the
    /// player's, so they're announced (the firmware's own setup isn't).
    live: bool,
}

impl SuperPedal {
    /// Hands the selected pedal its two playable knobs.
    fn apply_knobs(&mut self) {
        let (a, b) = (self.knobs[CTRL_1], self.knobs[CTRL_2]);
        if let Some(p) = self.rack.get(self.active) {
            p.set_knob(0, a);
            p.set_knob(1, b);
        }
    }

    /// Starts the target pedal (the old one is silent by now).
    fn start_target(&mut self) {
        self.active = self.target;
        self.trim = db(TRIM_DB[self.active]);
        self.pool.reset();
        let pool = &mut self.pool;
        if let Some(p) = self.rack.get(self.active) {
            p.restart(pool);
            // Its own right knob (mostly Level) stays at its default.
            let d = p.default_knob(2);
            p.set_knob(2, d);
        }
        self.apply_knobs();
        self.phase = Phase::Clearing(0);
    }

    /// The position playing (0 = no pedal), for the host demo.
    #[cfg(not(target_os = "none"))]
    #[allow(dead_code)]
    pub fn active(&self) -> usize {
        self.active
    }

    /// Whether a pedal change is under way, for the host demo.
    #[cfg(not(target_os = "none"))]
    #[allow(dead_code)]
    pub fn switching(&self) -> bool {
        self.phase != Phase::Playing || self.active != self.target
    }
}

impl Pedal for SuperPedal {
    const PARAMS: [Param; 3] =
        [Param { name: "Pedal", default: 0.0 }, Param { name: "Ctrl 1", default: 0.6 }, Param { name: "Ctrl 2", default: 0.5 }];
    /// The pedals' memory comes from the firmware's working buffer.
    const POOL: usize = 0;

    const NEW: Self = Self {
        knobs: [Self::PARAMS[0].default, Self::PARAMS[1].default, Self::PARAMS[2].default],
        selector: Selector::new(POSITIONS),
        active: 0,
        target: 0,
        phase: Phase::Playing,
        fade: Smooth::new(1.0),
        trim: 1.0,
        rack: Rack::NEW,
        pool: Pool::EMPTY,
        has_pool: false,
        voice: Announcer::new(&PROMPTS),
        live: false,
    };

    fn init(&mut self, _: &mut Pool) {
        self.fade.time(FADE_S);
        self.voice.init(DUCK_S);
    }

    fn attach(&mut self, ptr: *mut f32, len: usize) {
        // SAFETY: the firmware's working buffer, ours for the patch's life.
        self.pool = unsafe { Pool::new(ptr, len) };
        self.has_pool = true;
    }

    fn set_knob(&mut self, idx: usize, v: f32) {
        self.knobs[idx] = v;
        match idx {
            PEDAL => {
                if self.live {
                    if let Some(z) = self.selector.update(v) {
                        self.target = z;
                        self.voice.say(z);
                    }
                } else {
                    self.selector.set(v);
                    self.target = self.selector.zone();
                }
            }
            CTRL_1 => {
                if let Some(p) = self.rack.get(self.active) {
                    p.set_knob(0, v);
                }
            }
            _ => {
                if let Some(p) = self.rack.get(self.active) {
                    p.set_knob(1, v);
                }
            }
        }
    }

    fn press(&mut self) {
        if self.phase == Phase::Playing {
            if let Some(p) = self.rack.get(self.active) {
                p.press();
            }
        }
    }

    fn hold(&mut self) {
        if self.phase == Phase::Playing {
            if let Some(p) = self.rack.get(self.active) {
                p.hold();
            }
        }
    }

    fn block(&mut self) {
        self.live = true;
        match self.phase {
            Phase::Playing => {
                if self.target != self.active {
                    self.phase = Phase::Fading;
                    self.fade.set(0.0);
                }
            }
            Phase::Fading => {
                if self.target == self.active {
                    // Turned back before the change happened.
                    self.phase = Phase::Playing;
                    self.fade.set(1.0);
                } else if self.fade.value() < 1e-3 && (self.has_pool || self.target == 0) {
                    self.start_target();
                }
            }
            Phase::Clearing(from) => {
                let to = from + CLEAR_PER_BLOCK;
                self.pool.clear(from, to.min(self.pool.used()));
                if to >= self.pool.used() {
                    self.phase = Phase::Playing;
                    self.fade.set(1.0);
                } else {
                    self.phase = Phase::Clearing(to);
                }
            }
        }
        if self.phase == Phase::Playing {
            if let Some(p) = self.rack.get(self.active) {
                p.block();
            }
        }
    }

    #[inline]
    fn tick(&mut self, l: f32, r: f32) -> (f32, f32) {
        let g = self.fade.tick();
        let (pl, pr) = match (self.phase, self.active) {
            (Phase::Clearing(_), _) => (0.0, 0.0),
            // No pedal: the dry guitar, untouched, to both outputs.
            (_, 0) => (l, l),
            (_, a) => {
                let trim = self.trim;
                match self.rack.get(a) {
                    Some(p) => {
                        let (pl, pr) = p.tick(l, r);
                        (pl * trim, pr * trim)
                    }
                    None => (l, l),
                }
            }
        };
        // The pedal's name goes on top, with the music ducked under it.
        let v = self.voice.speak() * VOICE_LEVEL;
        let duck = 1.0 - DUCK * self.voice.duck_step();
        (soft_limit(pl * g * duck + v, LIMIT_KNEE), soft_limit(pr * g * duck + v, LIMIT_KNEE))
    }

    fn led(&self) -> Color {
        match self.rack.get_ref(self.active) {
            Some(p) => p.led(),
            None => Color::DimWhite,
        }
    }
}
