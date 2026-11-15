//! The whole patch: the Les Paul's volume pot -> Echoplex preamp -> Tone
//! Bender (crossfaded in) -> Plexi -> 4x12 -> Echoplex tape echo, all heard
//! by close and distant room mics (the room squashed by a compressor).

use super::echoplex_pre::EchoplexPre;
use super::plexi::Plexi;
use super::presets::*;
use super::room::Room;
use super::tape_echo::TapeEcho;
use super::tone_bender::ToneBender;
use crate::pedalkit::abi::Color;
use crate::pedalkit::dsp::{Biquad, db};
use crate::pedalkit::env::Follower;
use crate::pedalkit::eq::EqChain;
use crate::pedalkit::gate::Gate;
use crate::pedalkit::helpers::{Smooth, limit};
use crate::pedalkit::pedal::{Param, Pedal};
use crate::pedalkit::pool::Pool;

/// Knob indices.
const GAIN: usize = 0;
const ROOM: usize = 1;
const LEVEL: usize = 2;

pub struct Patch {
    knobs: [f32; 3],
    fuzz_on: bool,
    echo_on: bool,
    pot: Smooth,
    mk: Smooth,
    out: Smooth,
    wet: Smooth,
    fuzz_mix: Smooth,
    echo_send: Smooth,
    hp_in: Biquad,
    pot_lp: Biquad,
    gate: Gate,
    pre: EchoplexPre,
    tb: ToneBender,
    plexi: Plexi,
    cab: EqChain<6>,
    echo: TapeEcho,
    room: Room,
    comp: Follower,
}

impl Patch {
    fn retarget(&mut self) {
        let g = self.knobs[GAIN];
        self.pot.set(db(POT_FLOOR_DB * (1.0 - g)));
        self.mk.set(db(GAIN_MAKEUP_DB * (GAIN_DEFAULT - g).max(0.0)));
        self.pot_lp.lowpass(POT_LP_MIN + POT_LP_RANGE * g, 0.6);
        self.wet.set(self.knobs[ROOM]);
        // Square law: 0 at minimum, unity at 3 o'clock, +5 dB at maximum.
        let v = self.knobs[LEVEL] / 0.75;
        self.out.set(v * v * OUT_TRIM);
    }

    /// Sets the footswitch states directly (host-side analysis only).
    #[cfg(not(target_os = "none"))]
    #[allow(dead_code)]
    pub fn set_switches(&mut self, fuzz: bool, echo: bool) {
        self.fuzz_on = fuzz;
        self.echo_on = echo;
        self.fuzz_mix.on(fuzz);
        self.echo_send.on(echo);
    }
}

impl Pedal for Patch {
    const PARAMS: [Param; 3] = [
        // The guitar's volume knob.
        Param { name: "Gain", default: GAIN_DEFAULT },
        // Close mic to distant room mics.
        Param { name: "Room", default: 0.4 },
        // Output; unity at 3 o'clock.
        Param { name: "Level", default: 0.75 },
    ];
    const POOL: usize = 32768 + 4096 + 2 * (677 + 719 + 773 + 821 + 347 + 263) + 6 * 23;

    const NEW: Self = Self {
        knobs: [Self::PARAMS[0].default, Self::PARAMS[1].default, Self::PARAMS[2].default],
        fuzz_on: false,
        echo_on: false,
        pot: Smooth::new(1.0),
        mk: Smooth::new(1.0),
        out: Smooth::new(0.0),
        wet: Smooth::new(0.0),
        fuzz_mix: Smooth::new(0.0),
        echo_send: Smooth::new(0.0),
        hp_in: Biquad::NEW,
        pot_lp: Biquad::NEW,
        gate: Gate::NEW,
        pre: EchoplexPre::NEW,
        tb: ToneBender::NEW,
        plexi: Plexi::NEW,
        cab: EqChain::NEW,
        echo: TapeEcho::NEW,
        room: Room::NEW,
        comp: Follower::NEW,
    };

    fn init(&mut self, pool: &mut Pool) {
        self.hp_in.highpass(70.0, 0.707);
        self.gate.init(&GATE);
        self.pre.init();
        self.tb.init();
        self.plexi.init();
        self.cab.init(&CAB);
        self.echo.init(pool);
        self.room.init(pool);
        self.comp.times(0.005, 0.15);
        for s in [&mut self.pot, &mut self.mk, &mut self.out, &mut self.wet] {
            s.time(0.02);
        }
        self.fuzz_mix.time(0.012);
        self.echo_send.time(0.012);
        self.retarget();
        for s in [&mut self.pot, &mut self.mk, &mut self.out, &mut self.wet] {
            s.snap();
        }
        self.fuzz_mix.on(self.fuzz_on);
        self.echo_send.on(self.echo_on);
        self.fuzz_mix.snap();
        self.echo_send.snap();
    }

    fn set_knob(&mut self, idx: usize, v: f32) {
        self.knobs[idx] = v;
        self.retarget();
    }

    /// Footswitch press: Tone Bender fuzz on/off.
    fn press(&mut self) {
        self.fuzz_on = !self.fuzz_on;
        self.fuzz_mix.on(self.fuzz_on);
    }

    /// Footswitch hold: Echoplex echo on/off.
    fn hold(&mut self) {
        self.echo_on = !self.echo_on;
        self.echo_send.on(self.echo_on);
    }

    #[inline]
    fn tick(&mut self, x: f32, _: f32) -> (f32, f32) {
        let pot = self.pot.tick();
        let mk = self.mk.tick();
        let out = self.out.tick();
        let wet = self.wet.tick();
        let fm = self.fuzz_mix.tick();
        let es = self.echo_send.tick();
        let x = self.hp_in.process(x);
        let gate = self.gate.process(x);
        // Guitar volume pot, then the Echoplex preamp; the Tone Bender is crossfaded in front of the amp.
        let x = self.pot_lp.process(x) * pot;
        let x = self.pre.process(x);
        let f = self.tb.process(x) * gate;
        let x = x + (f - x) * fm;
        let g = self.cab.process(self.plexi.process(x)) * mk;
        // The echo's send is switched, so repeats ring out after it's turned off.
        let e = self.echo.process(g * es) * ECHO_MIX;
        let dl = g + e * ECHO_PAN.0;
        let dr = g + e * ECHO_PAN.1;
        // Room mics hear everything, squashed hard by a compressor.
        let (rl, rr) = self.room.process(0.5 * (dl + dr));
        let ce = self.comp.follow(rl.abs().max(rr.abs()));
        let cg = ROOM_GAIN / (1.0 + ROOM_COMP * ce);
        let dry = 1.0 - ROOM_DRY_DUCK * wet;
        let l = dl * dry + rl * cg * wet;
        let r = dr * dry + rr * cg * wet;
        (limit(l * out), limit(r * out))
    }

    fn led(&self) -> Color {
        match (self.fuzz_on, self.echo_on) {
            (false, false) => Color::LightYellow,
            (false, true) => Color::LightBlue,
            (true, false) => Color::Red,
            (true, true) => Color::Magenta,
        }
    }
}
