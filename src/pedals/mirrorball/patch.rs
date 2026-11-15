//! The whole patch. Two voices run all the time and the footswitch
//! crossfades between them:
//!   Peace     the tweed turned down
//!   Downtown  the tweed cranked, with a sustainer that lets held notes bloom
//! both after the Bigsby, then an open-back 1x12 and a live room.

use super::bigsby::Bigsby;
use super::downtown::Downtown;
use super::peace::Peace;
use super::presets::*;
use super::sustainer::Sustainer;
use crate::pedalkit::abi::Color;
use crate::pedalkit::dsp::{Biquad, db};
use crate::pedalkit::eq::EqChain;
use crate::pedalkit::helpers::{Smooth, limit};
use crate::pedalkit::pedal::{Param, Pedal};
use crate::pedalkit::pool::Pool;
use crate::pedalkit::reverb::Reverb;

/// Knob indices.
const DRIVE: usize = 0;
const ROOM: usize = 1;
const LEVEL: usize = 2;

pub struct Patch {
    knobs: [f32; 3],
    loud: bool,
    bigsby_on: bool,
    g_pre: Smooth,
    out: Smooth,
    room: Smooth,
    mix: Smooth,
    hp_in: Biquad,
    sustainer: Sustainer,
    bigsby: Bigsby,
    downtown: Downtown,
    peace: Peace,
    cab: EqChain<6>,
    rev: Reverb,
}

impl Patch {
    fn retarget(&mut self) {
        self.g_pre.set(db(DRIVE_MIN_DB + DRIVE_RANGE_DB * self.knobs[DRIVE]));
        // Square law: 0 at minimum, unity at 3 o'clock, +5 dB at maximum.
        let v = self.knobs[LEVEL] / 0.75;
        self.out.set(v * v * OUT_TRIM);
        self.room.set(self.knobs[ROOM]);
        self.mix.on(self.loud);
        self.bigsby.on(self.bigsby_on);
    }

    /// The sustainer's fade (0..1), for host-side analysis.
    #[cfg(not(target_os = "none"))]
    #[allow(dead_code)]
    pub fn fb_ramp(&self) -> f32 {
        self.sustainer.ramp
    }

    /// Sets the footswitch states directly and keeps the sustainer from
    /// latching (host-side analysis only).
    #[cfg(not(target_os = "none"))]
    #[allow(dead_code)]
    pub fn analysis(&mut self, loud: bool, bigsby: bool) {
        self.loud = loud;
        self.bigsby_on = bigsby;
        self.retarget();
        self.sustainer.since_onset = 0;
    }
}

impl Pedal for Patch {
    const PARAMS: [Param; 3] = [
        // Preamp gain, 0 to 38 dB: edge of breakup at 0, Downtown at the default, Ocean at max.
        Param { name: "Drive", default: DRIVE_DEFAULT },
        // Room size and level; CCW is dry.
        Param { name: "Room", default: 0.45 },
        // Output; unity at 3 o'clock.
        Param { name: "Level", default: 0.75 },
    ];
    const POOL: usize = 128 + 2048 + 2 * (1214 + 1293 + 1389 + 1475 + 1547 + 1622 + 1694 + 1759 + 605 + 480 + 371 + 245) + 12 * 25;

    const NEW: Self = Self {
        knobs: [Self::PARAMS[0].default, Self::PARAMS[1].default, Self::PARAMS[2].default],
        loud: true,
        bigsby_on: false,
        g_pre: Smooth::new(1.0),
        out: Smooth::new(0.0),
        room: Smooth::new(Self::PARAMS[ROOM].default),
        mix: Smooth::new(1.0),
        hp_in: Biquad::NEW,
        sustainer: Sustainer::NEW,
        bigsby: Bigsby::NEW,
        downtown: Downtown::NEW,
        peace: Peace::NEW,
        cab: EqChain::NEW,
        rev: Reverb::NEW,
    };

    fn init(&mut self, pool: &mut Pool) {
        self.hp_in.highpass(70.0, 0.707);
        self.sustainer.init();
        self.bigsby.init(pool);
        self.downtown.init();
        self.peace.init();
        self.cab.init(&CAB);
        self.rev.init(&REVERB, pool);
        for s in [&mut self.g_pre, &mut self.out, &mut self.room] {
            s.time(0.02);
        }
        self.mix.time(0.012);
        self.retarget();
        self.g_pre.snap();
        self.out.snap();
        self.room.snap();
        self.mix.snap();
    }

    fn set_knob(&mut self, idx: usize, v: f32) {
        self.knobs[idx] = v;
        self.retarget();
    }

    /// Footswitch press: Peace <-> Downtown.
    fn press(&mut self) {
        self.loud = !self.loud;
        self.retarget();
    }

    /// Footswitch hold: Bigsby wobble on/off (both voices).
    fn hold(&mut self) {
        self.bigsby_on = !self.bigsby_on;
        self.retarget();
    }

    #[inline]
    fn tick(&mut self, x: f32, _: f32) -> (f32, f32) {
        let g = self.g_pre.tick();
        let out = self.out.tick();
        let rs = self.room.tick();
        let m = self.mix.tick();
        let x = self.hp_in.process(x);
        self.sustainer.track(x);
        // The Bigsby is on the guitar, so it moves both voices.
        let x = self.bigsby.process(x);
        let xs = self.sustainer.process(x, self.loud);
        // Both voices always run, so switching is a click-free crossfade.
        let o = self.downtown.process(xs, g);
        let p = self.peace.process(x) * db(PEACE_LEVEL_DB + PEACE_MAKEUP_DB);
        let y = self.cab.process(p + (o - p) * m);
        // Shared room, so its tail carries across a switch.
        let (wl, wr) = self.rev.process(y, ROOM_MIN + ROOM_RANGE * rs);
        let wet = rs * (PEACE_WET + (OCEAN_WET - PEACE_WET) * m);
        (limit((y + wet * wl) * out), limit((y + wet * wr) * out))
    }

    fn led(&self) -> Color {
        match (self.loud, self.bigsby_on) {
            (false, false) => Color::LightGreen,
            (false, true) => Color::LightYellow,
            (true, false) => Color::DarkCobalt,
            (true, true) => Color::Beige,
        }
    }
}
