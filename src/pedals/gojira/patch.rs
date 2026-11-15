//! The whole patch. Two voices run all the time and the footswitch
//! crossfades between them:
//!   Magma   clean amp with gentle compression, a stereo chorus, a dotted-
//!           eighth echo and a big room (the ambient Gojira cleans)
//!   Sirius  a tight overdrive boost into a two-stage high-gain preamp, a
//!           deep, slightly scooped tone stack and a power-amp squash, behind
//!           a fast noise gate for the staccato stops
//! then a closed-back 4x12 cab sim and a stereo reverb shared by both. The
//! Whammy (octave up, before the amp) sounds in Sirius only.

use super::presets::*;
use super::sirius::Sirius;
use super::space::Space;
use super::whammy::Whammy;
use crate::pedalkit::abi::Color;
use crate::pedalkit::clean::CleanAmp;
use crate::pedalkit::dsp::{Biquad, db};
use crate::pedalkit::env::Onset;
use crate::pedalkit::eq::EqChain;
use crate::pedalkit::gate::Gate;
use crate::pedalkit::helpers::{Smooth, limit};
use crate::pedalkit::pedal::{Param, Pedal};
use crate::pedalkit::pool::Pool;
use crate::pedalkit::reverb::Reverb;

/// Knob indices.
const GAIN: usize = 0;
const SPACE: usize = 1;
const LEVEL: usize = 2;

pub struct Patch {
    knobs: [f32; 3],
    loud: bool,
    whammy_on: bool,
    g_pre: Smooth,
    out: Smooth,
    space: Smooth,
    mix: Smooth,
    hp_in: Biquad,
    gate: Gate,
    onset: Onset,
    whammy: Whammy,
    sirius: Sirius,
    clean: CleanAmp,
    clean_eq: EqChain<2>,
    cab: EqChain<6>,
    fx: Space,
    rev: Reverb,
}

impl Patch {
    fn retarget(&mut self) {
        self.g_pre.set(db(GAIN_MIN_DB + GAIN_RANGE_DB * self.knobs[GAIN]));
        // Square law: 0 at minimum, unity at 3 o'clock, +5 dB at maximum.
        let v = self.knobs[LEVEL] / 0.75;
        self.out.set(v * v * OUT_TRIM);
        self.space.set(self.knobs[SPACE]);
        self.mix.on(self.loud);
    }
}

impl Pedal for Patch {
    const PARAMS: [Param; 3] = [
        // Preamp gain, ~22 to 46 dB into a fixed second stage.
        Param { name: "Gain", default: 0.7 },
        // Chorus, echo and reverb; CCW = dry.
        Param { name: "Space", default: 0.5 },
        // Output; unity at 3 o'clock.
        Param { name: "Level", default: 0.75 },
    ];
    const POOL: usize =
        4096 + 1024 + 32768 + 2048 + 2 * (1214 + 1293 + 1389 + 1475 + 1547 + 1622 + 1694 + 1759 + 605 + 480 + 371 + 245) + 12 * 25;

    const NEW: Self = Self {
        knobs: [Self::PARAMS[0].default, Self::PARAMS[1].default, Self::PARAMS[2].default],
        loud: true,
        whammy_on: false,
        g_pre: Smooth::new(1.0),
        out: Smooth::new(0.0),
        space: Smooth::new(Self::PARAMS[SPACE].default),
        mix: Smooth::new(1.0),
        hp_in: Biquad::NEW,
        gate: Gate::NEW,
        onset: Onset::NEW,
        whammy: Whammy::NEW,
        sirius: Sirius::NEW,
        clean: CleanAmp::NEW,
        clean_eq: EqChain::NEW,
        cab: EqChain::NEW,
        fx: Space::NEW,
        rev: Reverb::NEW,
    };

    fn init(&mut self, pool: &mut Pool) {
        self.hp_in.highpass(80.0, 0.707);
        self.gate.init(&GATE);
        self.onset.time(0.03);
        self.whammy.init(pool);
        self.sirius.init();
        self.clean.init(&CLEAN);
        self.clean_eq.init(&CLEAN_EQ);
        self.cab.init(&CAB);
        self.fx.init(pool);
        self.rev.init(&REVERB, pool);
        for s in [&mut self.g_pre, &mut self.out, &mut self.space] {
            s.time(0.02);
        }
        self.mix.time(0.012);
        self.retarget();
        self.g_pre.snap();
        self.out.snap();
        self.space.snap();
        self.mix.snap();
    }

    fn set_knob(&mut self, idx: usize, v: f32) {
        self.knobs[idx] = v;
        self.retarget();
    }

    /// Footswitch press: Magma <-> Sirius.
    fn press(&mut self) {
        self.loud = !self.loud;
        self.retarget();
    }

    /// Footswitch hold: Whammy armed/off (sounds in Sirius only).
    fn hold(&mut self) {
        self.whammy_on = !self.whammy_on;
        self.retarget();
    }

    #[inline]
    fn tick(&mut self, x: f32, _: f32) -> (f32, f32) {
        let g = self.g_pre.tick();
        let out = self.out.tick();
        let sp = self.space.tick();
        let m = self.mix.tick();
        let x = self.hp_in.process(x);
        let gate = self.gate.process(x);
        let env = self.gate.env.env;
        self.onset.update(env);
        let xd = self.whammy.process(x, env, self.onset.slow, self.whammy_on && self.loud);
        // Both voices always run, so switching is a click-free crossfade.
        let d = self.sirius.process(xd, g) * gate;
        let c = self.clean_eq.process(self.clean.process(x)) * db(MAGMA_LEVEL_DB + CLEAN_MAKEUP_DB);
        let y = self.cab.process(c + (d - c) * m);
        // Stereo chorus on the clean, then the echo and the shared reverb (so
        // its tail carries across a switch).
        let (yl, yr) = self.fx.chorus(y, sp, m);
        let e = self.fx.echo(0.5 * (yl + yr) * ECHO_SEND * sp * (1.0 - m));
        let (wl, wr) = self.rev.process(0.5 * (yl + yr) + e, ROOM_MIN + ROOM_RANGE * sp);
        let wet = sp * (MAGMA_WET + (SIRIUS_WET - MAGMA_WET) * m);
        (limit((yl + e + wet * wl) * out), limit((yr + e + wet * wr) * out))
    }

    fn led(&self) -> Color {
        match (self.loud, self.whammy_on) {
            (false, false) => Color::LightBlue,
            (false, true) => Color::DimCyan,
            (true, false) => Color::Blue,
            (true, true) => Color::Magenta,
        }
    }
}
