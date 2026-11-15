//! Tony Iommi, 1970-73: SG -> Dallas Rangemaster (mid-modded) -> Laney
//! Supergroup 100 W cranked -> 4x12. Mono signal path (L+R summed), stereo
//! output. Holding the footswitch turns it into the *Planet Caravan* Leslie.
//!
//!   Knob L  GAIN   Rangemaster boost + Laney preamp drive (Caravan: rotor depth)
//!   Knob M  TONE   tone stack morph, dark/sludge <-> bright/cutting (Caravan: rotor speed)
//!   Knob R  LEVEL  output volume
//!   Press   LEAD   lead boost on/off (extra gain + mid push)
//!   Hold    PLANET CARAVAN mode on/off
//!   LED     dark red = rhythm, red = lead, a step brighter while the clipping
//!           stages are hit hard; Caravan blue / dim blue in time with the rotor

use super::caravan::Caravan;
use super::heavy::Heavy;
use super::presets::*;
use crate::pedalkit::abi::Color;
use crate::pedalkit::env::ExpEnvelope;
use crate::pedalkit::helpers::soft_limit;
use crate::pedalkit::pedal::{Param, Pedal};
use crate::pedalkit::pool::Pool;

/// Knob indices.
const GAIN: usize = 0;
const TONE: usize = 1;
const LEVEL: usize = 2;

pub struct Patch {
    knobs: [f32; 3],
    level: f32,
    planet_caravan: bool,
    heavy: Heavy,
    caravan: Caravan,
    /// Drives the LED's brightness step.
    hot: ExpEnvelope,
}

impl Patch {
    fn apply(&mut self) {
        self.heavy.set_gain(self.knobs[GAIN]);
        self.caravan.set_depth(self.knobs[GAIN]);
        self.heavy.set_tone(self.knobs[TONE]);
        self.caravan.set_rate(self.knobs[TONE]);
        let l = self.knobs[LEVEL];
        self.level = l * l * LEVEL_MAX;
    }

    fn reset_state(&mut self) {
        self.heavy.reset();
        self.caravan.reset();
        self.hot.reset();
    }

    #[inline]
    fn output(&self, x: f32) -> f32 {
        soft_limit(x * self.level, LIMIT_KNEE).clamp(-1.0, 1.0)
    }
}

impl Pedal for Patch {
    const PARAMS: [Param; 3] = [
        // Never clean: 0 is Paranoid crunch, 1 is Sweet Leaf wall.
        Param { name: "Gain", default: 0.65 },
        // Left dark/sludge, centre ~Master of Reality, right bright/cutting.
        Param { name: "Tone", default: 0.5 },
        // Output volume, soft-limited at the top.
        Param { name: "Level", default: 0.7 },
    ];
    const POOL: usize = 2 * super::leslie::DELAY_SIZE;

    const NEW: Self = Self {
        knobs: [Self::PARAMS[0].default, Self::PARAMS[1].default, Self::PARAMS[2].default],
        level: 1.0,
        planet_caravan: false,
        heavy: Heavy::NEW,
        caravan: Caravan::NEW,
        hot: ExpEnvelope::NEW,
    };

    fn init(&mut self, pool: &mut Pool) {
        self.heavy.init();
        self.caravan.init(pool);
        self.hot.set(LED_ATTACK_MS, LED_RELEASE_MS, FS);
        self.apply();
        self.reset_state();
    }

    fn set_knob(&mut self, idx: usize, v: f32) {
        self.knobs[idx] = v;
        self.apply();
    }

    /// Footswitch press: lead boost on/off.
    fn press(&mut self) {
        self.heavy.lead = !self.heavy.lead;
        self.apply();
    }

    /// Footswitch hold: Planet Caravan on/off.
    fn hold(&mut self) {
        self.planet_caravan = !self.planet_caravan;
        self.reset_state();
    }

    #[inline]
    fn tick(&mut self, l: f32, r: f32) -> (f32, f32) {
        let x = (l + r) * 0.5;
        if self.planet_caravan {
            let (cl, cr) = self.caravan.process(x);
            (self.output(cl), self.output(cr))
        } else {
            let y = self.heavy.process(x);
            self.hot.process(y);
            let o = self.output(y * OUT_TRIM);
            (o, o)
        }
    }

    fn led(&self) -> Color {
        if self.planet_caravan {
            return if self.caravan.leslie.phase < 0.5 { Color::Blue } else { Color::DimBlue };
        }
        let hot = self.hot.env > LED_HOT_THRESH;
        match (self.heavy.lead, hot) {
            (true, true) => Color::LightYellow,
            (true, false) => Color::Red,
            (false, true) => Color::Red,
            (false, false) => Color::DarkRed,
        }
    }
}
