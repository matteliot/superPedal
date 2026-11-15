//! The whole patch. Two voices run all the time and the footswitch
//! crossfades between them:
//!   Honey        a bright, jangly clean with gentle compression
//!   Psychocandy  fuzz into fuzz, with an optional feedback squall
//! then a small open-back combo cab and a big shared reverb.

use super::fuzz::Fuzz;
use super::presets::*;
use crate::pedalkit::abi::Color;
use crate::pedalkit::clean::CleanAmp;
use crate::pedalkit::dsp::{Biquad, SR, db, exp2, sin};
use crate::pedalkit::eq::EqChain;
use crate::pedalkit::feedback::Feedback;
use crate::pedalkit::gate::Gate;
use crate::pedalkit::helpers::{Phase, Smooth, limit};
use crate::pedalkit::pedal::{Param, Pedal};
use crate::pedalkit::pool::Pool;
use crate::pedalkit::reverb::Reverb;
use core::f32::consts::TAU;

/// Knob indices.
const FUZZ: usize = 0;
const REVERB_KNOB: usize = 1;
const LEVEL: usize = 2;

pub struct Patch {
    knobs: [f32; 3],
    loud: bool,
    feedback: bool,
    g_fuzz: Smooth,
    out: Smooth,
    rev: Smooth,
    mix: Smooth,
    hp_in: Biquad,
    gate: Gate,
    fuzz: Fuzz,
    fb: Feedback,
    fb_a: Phase,
    fb_b: Phase,
    fb_count: u32,
    clean: CleanAmp,
    clean_eq: EqChain<2>,
    cab: EqChain<6>,
    reverb: Reverb,
}

impl Patch {
    fn retarget(&mut self) {
        self.g_fuzz.set(db(FUZZ_MIN_DB + FUZZ_RANGE_DB * self.knobs[FUZZ]));
        // Square law: 0 at minimum, unity at 3 o'clock, +5 dB at maximum.
        let v = self.knobs[LEVEL] / 0.75;
        self.out.set(v * v * OUT_TRIM);
        self.rev.set(self.knobs[REVERB_KNOB]);
        self.mix.on(self.loud);
    }
}

impl Pedal for Patch {
    const PARAMS: [Param; 3] = [
        // Fuzz gain, ~20 to 50 dB.
        Param { name: "Fuzz", default: 0.75 },
        // Wet level and room size: CCW dry, CW an endless wash.
        Param { name: "Reverb", default: 0.55 },
        // Output; unity at 3 o'clock.
        Param { name: "Level", default: 0.75 },
    ];
    const POOL: usize = 512 + 2048 + 2 * (1214 + 1293 + 1389 + 1475 + 1547 + 1622 + 1694 + 1759 + 605 + 480 + 371 + 245) + 12 * 25;

    const NEW: Self = Self {
        knobs: [Self::PARAMS[0].default, Self::PARAMS[1].default, Self::PARAMS[2].default],
        loud: true,
        feedback: false,
        g_fuzz: Smooth::new(1.0),
        out: Smooth::new(0.0),
        rev: Smooth::new(Self::PARAMS[REVERB_KNOB].default),
        mix: Smooth::new(1.0),
        hp_in: Biquad::NEW,
        gate: Gate::NEW,
        fuzz: Fuzz::NEW,
        fb: Feedback::NEW,
        fb_a: Phase::new(0.0),
        fb_b: Phase::new(0.4),
        fb_count: 0,
        clean: CleanAmp::NEW,
        clean_eq: EqChain::NEW,
        cab: EqChain::NEW,
        reverb: Reverb::NEW,
    };

    fn init(&mut self, pool: &mut Pool) {
        self.hp_in.highpass(80.0, 0.707);
        self.gate.init(&GATE);
        self.fuzz.init();
        self.fb.init(&FEEDBACK, pool);
        self.clean.init(&CLEAN);
        self.clean_eq.init(&CLEAN_EQ);
        self.cab.init(&CAB);
        self.reverb.init(&REVERB, pool);
        for s in [&mut self.g_fuzz, &mut self.out, &mut self.rev] {
            s.time(0.02);
        }
        self.mix.time(0.012);
        self.retarget();
        self.g_fuzz.snap();
        self.out.snap();
        self.rev.snap();
        self.mix.snap();
    }

    fn set_knob(&mut self, idx: usize, v: f32) {
        self.knobs[idx] = v;
        self.retarget();
    }

    /// Footswitch press: Honey <-> Psychocandy.
    fn press(&mut self) {
        self.loud = !self.loud;
        self.retarget();
    }

    /// Footswitch hold: feedback squall armed/off (Psychocandy only).
    fn hold(&mut self) {
        self.feedback = !self.feedback;
        self.retarget();
    }

    #[inline]
    fn tick(&mut self, x: f32, _: f32) -> (f32, f32) {
        let g = self.g_fuzz.tick();
        let out = self.out.tick();
        let rs = self.rev.tick();
        let m = self.mix.tick();
        let x = self.hp_in.process(x);
        let gate = self.gate.process(x);
        // Squall: its band-pass wanders over ~1.5 to 3.6 kHz on two slow
        // sines, so the screech changes pitch by itself. Retuned every 32
        // samples. Injection is normalised by the first fuzz stage's gain so
        // the bloom is the same at any Fuzz setting. As it blooms the squall
        // takes the string over (the amp is driving it), so the picked sound
        // is ducked under it; otherwise the much louder string would hog the
        // clippers and smother the feedback.
        let pa = self.fb_a.step(FB_RATE_A / SR);
        let pb = self.fb_b.step(FB_RATE_B / SR);
        self.fb_count = self.fb_count.wrapping_add(1);
        if self.fb_count & 31 == 0 {
            let w = 0.6 * sin(TAU * pa) + 0.4 * sin(TAU * pb);
            self.fb.tune(FB_FREQ * exp2(FB_WANDER * w));
        }
        self.fb.drive(self.feedback && self.loud && self.gate.open);
        let fb = self.fb.read(g);
        let duck = 1.0 - FB_DUCK * self.fb.ramp;
        // Both voices always run, so switching is a click-free crossfade.
        let d = self.fuzz.process(x * duck + fb, g) * gate;
        self.fb.send(d);
        let c = self.clean_eq.process(self.clean.process(x)) * db(HONEY_LEVEL_DB + CLEAN_MAKEUP_DB);
        let y = self.cab.process(c + (d - c) * m);
        // Shared reverb, so its tail carries across a switch.
        let (wl, wr) = self.reverb.process(y, ROOM_MIN + ROOM_RANGE * rs);
        let wet = rs * (HONEY_WET + (PSYCHO_WET - HONEY_WET) * m);
        (limit((y + wet * wl) * out), limit((y + wet * wr) * out))
    }

    fn led(&self) -> Color {
        match (self.loud, self.feedback) {
            (false, false) => Color::LightYellow,
            (false, true) => Color::Beige,
            (true, false) => Color::Magenta,
            (true, true) => Color::Red,
        }
    }
}
