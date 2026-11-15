//! The whole patch: Muff -> wah -> one guitar into two stacks at once, one per
//! side (no gate: the wall hums and rings out) -> a 4x12 per side -> level.

use crate::pedalkit::abi::Color;
use crate::pedalkit::amp::Amp;
use crate::pedalkit::dsp::db;
use crate::pedalkit::eq::EqChain;
use crate::pedalkit::helpers::{Smooth, limit};
use crate::pedalkit::muff::Muff;
use crate::pedalkit::pedal::{Param, Pedal};
use crate::pedalkit::pool::Pool;
use crate::pedalkit::wah::Wah;
use super::presets::{CAB_4X12, MATAMP, MUFF, ORANGE, WAH};

/// Knob indices.
const GAIN: usize = 0;
const TONE: usize = 1;
const LEVEL: usize = 2;

/// Preamp gain (dB) at knob 0, and the extra at knob 1.
const GAIN_MIN_DB: f32 = 14.0;
const GAIN_RANGE_DB: f32 = 30.0;
/// Lead: a little more level (dB) so the solo sits on top.
const LEAD_LEVEL_DB: f32 = 2.5;
/// Output trim: unity Level puts the riff around -14 dB RMS.
const OUT_TRIM: f32 = 0.47;
/// Knob smoothing (s).
const KNOB_GLIDE: f32 = 0.02;

pub struct Patch {
    knobs: [f32; 3],
    lead: bool,
    muff_on: bool,
    gain: Smooth,
    out: Smooth,
    tone: Smooth,
    muff: Muff,
    wah: Wah,
    amp_l: Amp,
    amp_r: Amp,
    cab_l: EqChain<6>,
    cab_r: EqChain<6>,
}

impl Patch {
    /// Points the smoothed values at the knobs and switches. Safe before
    /// `init`, which re-applies them and snaps.
    fn retarget(&mut self) {
        let lead = if self.lead { 1.0 } else { 0.0 };
        self.gain.set(db(GAIN_MIN_DB + GAIN_RANGE_DB * self.knobs[GAIN]));
        // Square law: 0 at minimum, unity at 3 o'clock, +5 dB at maximum.
        let v = self.knobs[LEVEL] / 0.75;
        self.out.set(v * v * OUT_TRIM * db(LEAD_LEVEL_DB * lead));
        self.tone.set(self.knobs[TONE]);
        self.wah.on(self.lead);
        self.muff.on(self.muff_on);
    }
}

impl Pedal for Patch {
    const PARAMS: [Param; 3] = [
        // Preamp gain, ~14 to 44 dB.
        Param { name: "Gain", default: 0.7 },
        // Amp treble: CCW dark and woolly, CW cutting.
        Param { name: "Tone", default: 0.4 },
        // Output; unity at 3 o'clock.
        Param { name: "Level", default: 0.75 },
    ];
    const POOL: usize = 0;

    const NEW: Self = Self {
        knobs: [Self::PARAMS[0].default, Self::PARAMS[1].default, Self::PARAMS[2].default],
        lead: false,
        muff_on: false,
        gain: Smooth::new(1.0),
        out: Smooth::new(0.0),
        tone: Smooth::new(Self::PARAMS[TONE].default),
        muff: Muff::NEW,
        wah: Wah::NEW,
        amp_l: Amp::NEW,
        amp_r: Amp::NEW,
        cab_l: EqChain::NEW,
        cab_r: EqChain::NEW,
    };

    fn init(&mut self, _: &mut Pool) {
        self.amp_l.init(&ORANGE);
        self.amp_r.init(&MATAMP);
        self.muff.init(&MUFF);
        self.wah.init(&WAH);
        self.cab_l.init(&CAB_4X12);
        self.cab_r.init(&CAB_4X12);
        for s in [&mut self.gain, &mut self.out, &mut self.tone] {
            s.time(KNOB_GLIDE);
        }
        self.retarget();
        self.gain.snap();
        self.out.snap();
        self.tone.snap();
        self.muff.snap();
        self.wah.snap();
    }

    fn set_knob(&mut self, idx: usize, v: f32) {
        self.knobs[idx] = v;
        self.retarget();
    }

    /// Footswitch press: Riff <-> Lead.
    fn press(&mut self) {
        self.lead = !self.lead;
        self.retarget();
    }

    /// Footswitch hold: Muff on/off.
    fn hold(&mut self) {
        self.muff_on = !self.muff_on;
        self.retarget();
    }

    #[inline]
    fn tick(&mut self, x: f32, _: f32) -> (f32, f32) {
        let g = self.gain.tick();
        let out = self.out.tick();
        let t = self.tone.tick();
        let x = self.muff.process(x);
        let x = self.wah.process(x);
        // One guitar into both stacks at once.
        let l = self.cab_l.process(self.amp_l.process(x, g, t));
        let r = self.cab_r.process(self.amp_r.process(x, g, t));
        (limit(l * out), limit(r * out))
    }

    fn led(&self) -> Color {
        match (self.lead, self.muff_on) {
            (false, false) => Color::Beige,
            (true, false) => Color::LightGreen,
            (false, true) => Color::DarkRed,
            (true, true) => Color::DarkLime,
        }
    }
}
