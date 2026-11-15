//! The whole patch (mono in from the left input, stereo out):
//!   tight HPF -> noise gate -> honky pre-mid push -> 2-stage asymmetric fuzz
//!   -> Mids EQ (never scooped) -> 4x12 cab sim -> Leslie rotary -> level
//! with a controlled-feedback loop around the fuzz for Thayil-style howl.

use super::fuzz::Fuzz;
use super::presets::*;
use super::rotary::Rotary;
use crate::pedalkit::abi::Color;
use crate::pedalkit::dsp::{Biquad, coef, db};
use crate::pedalkit::eq::EqChain;
use crate::pedalkit::feedback::Feedback;
use crate::pedalkit::gate::Gate;
use crate::pedalkit::helpers::{Smooth, limit};
use crate::pedalkit::pedal::{Param, Pedal};
use crate::pedalkit::pool::Pool;

/// Knob indices.
const FUZZ: usize = 0;
const MIDS: usize = 1;
const LEVEL: usize = 2;

pub struct Patch {
    knobs: [f32; 3],
    mode: u8,
    feedback: bool,
    g1: Smooth,
    g2: Smooth,
    out: Smooth,
    wet: Smooth,
    hp_in: Biquad,
    gate: Gate,
    c_gate_close_bh: f32,
    gate_scale_bh: f32,
    fb: Feedback,
    fuzz: Fuzz,
    mid_eq: Biquad,
    cab: EqChain<6>,
    rotary: Rotary,
}

impl Patch {
    /// Recompute gain targets and the Mids EQ from the knobs and mode.
    fn retarget(&mut self) {
        let mut total = FUZZ_MIN_DB + FUZZ_RANGE_DB * self.knobs[FUZZ];
        if self.mode != HEAVY {
            total -= BLACK_HOLE_FUZZ_CUT_DB;
        }
        self.g1.set(db(total * 0.6));
        self.g2.set(db(total * 0.4));
        // Square law: 0 at minimum, unity at 3 o'clock, +5 dB at maximum.
        let v = self.knobs[LEVEL] / 0.75;
        self.out.set(v * v * OUT_TRIM * if self.mode == HEAVY { 1.0 } else { db(BLACK_HOLE_MAKEUP_DB) });
        self.mid_eq.peak(750.0, 0.8, -4.0 + 16.0 * self.knobs[MIDS]);
        self.wet.set(if self.mode == HEAVY { 0.0 } else { 1.0 });
    }
}

impl Pedal for Patch {
    const PARAMS: [Param; 3] = [
        // Gain into the fuzz, ~6 to 50 dB.
        Param { name: "Fuzz", default: 0.6 },
        // 750 Hz peak, -4 to +12 dB; noon = +4 dB, mid-heavy.
        Param { name: "Mids", default: 0.5 },
        // Output; unity at 3 o'clock.
        Param { name: "Level", default: 0.75 },
    ];
    const POOL: usize = 512 + 128 + 64;

    const NEW: Self = Self {
        knobs: [Self::PARAMS[0].default, Self::PARAMS[1].default, Self::PARAMS[2].default],
        mode: HEAVY,
        feedback: false,
        g1: Smooth::new(1.0),
        g2: Smooth::new(1.0),
        out: Smooth::new(0.0),
        wet: Smooth::new(0.0),
        hp_in: Biquad::NEW,
        gate: Gate::NEW,
        c_gate_close_bh: 1.0,
        gate_scale_bh: 1.0,
        fb: Feedback::NEW,
        fuzz: Fuzz::NEW,
        mid_eq: Biquad::NEW,
        cab: EqChain::NEW,
        rotary: Rotary::NEW,
    };

    fn init(&mut self, pool: &mut Pool) {
        self.hp_in.highpass(95.0, 0.707); // tight lows into the fuzz
        self.gate.init(&GATE);
        self.c_gate_close_bh = coef(BLACK_HOLE_GATE_FADE_S);
        self.gate_scale_bh = db(BLACK_HOLE_GATE_DB);
        self.fb.init(&FEEDBACK, pool);
        self.fuzz.init();
        self.cab.init(&CAB);
        self.rotary.init(pool);
        for s in [&mut self.g1, &mut self.g2, &mut self.out] {
            s.time(0.02);
        }
        self.wet.time(0.25);
        self.retarget();
        self.g1.snap();
        self.g2.snap();
        self.out.snap();
    }

    fn set_knob(&mut self, idx: usize, v: f32) {
        self.knobs[idx] = v;
        self.retarget();
    }

    /// Footswitch press: Heavy -> Black Hole (slow rotary) -> Black Hole fast -> Heavy.
    fn press(&mut self) {
        self.mode = (self.mode + 1) % MODES;
        self.retarget();
    }

    /// Footswitch hold: feedback on/off.
    fn hold(&mut self) {
        self.feedback = !self.feedback;
        self.retarget();
    }

    #[inline]
    fn tick(&mut self, x: f32, _: f32) -> (f32, f32) {
        let g1 = self.g1.tick();
        let g2 = self.g2.tick();
        let out = self.out.tick();
        let wet = self.wet.tick();
        let x = self.hp_in.process(x);
        // Gate on the input envelope; lower and gentler in the Black Hole modes.
        let (scale, fade) =
            if self.mode == HEAVY { (1.0, self.gate.c_out()) } else { (self.gate_scale_bh, self.c_gate_close_bh) };
        let gate = self.gate.process_with(x, scale, fade);
        // Feedback blooms while a note sustains and dies as soon as it's muted.
        // Injection is normalised by the fuzz gain so the loop gain (and so
        // the bloom) is the same at any Fuzz setting.
        self.fb.drive(self.feedback && self.gate.open);
        let fb = self.fb.read(g1 * g2);
        let y = self.fuzz.process(x + fb, g1, g2) * gate;
        self.fb.send(y);
        let y = self.cab.process(self.mid_eq.process(y));
        let (rl, rr) = self.rotary.process(y, self.mode == BLACK_HOLE_FAST);
        let l = (y + (rl - y) * wet) * out;
        let r = (y + (rr - y) * wet) * out;
        (limit(l), limit(r))
    }

    fn led(&self) -> Color {
        match (self.mode, self.feedback) {
            (HEAVY, false) => Color::Red,
            (HEAVY, true) => Color::LightYellow,
            (BLACK_HOLE, false) => Color::DimBlue,
            (BLACK_HOLE, true) => Color::LightBlue,
            (_, false) => Color::Magenta,
            (_, true) => Color::PastelGreen,
        }
    }
}
