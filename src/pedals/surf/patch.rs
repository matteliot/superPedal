//! The most surf pedal possible, meant to sit in front of a Fender Twin. No
//! amp model; the Twin is real. Everything else that made the records:
//! bright single-coil "spank", a tape slapback, a 6G15 outboard tank with a
//! driver tube and two dispersive springs (the chirp that reads as drip), a
//! footswitch you can slam, and a vibrato arm you can push with your foot.
//!
//!   Knob L      DRIP     the reverb: drives the springs harder and mixes more of them in, together
//!   Knob M      ECHO     tape slapback level and repeats (0 = off); time set by the flavour
//!   Knob R      SPANK    Jaguar/Jazzmaster top end and pick attack in front of everything
//!   Tap         CRASH    slam the tank: noise splash + thud through the springs
//!   Tap-tap     DIVE     a second tap inside 400 ms pushes the vibrato arm
//!   Hold        FLAVOUR  Drenched (cranked 6G15, short slap) <-> Clean (shorter tank, longer echo)
//!   LED         blue (Drenched) / dim blue (Clean), lighter while the springs ring hard;
//!               white flash on a crash, magenta during a dive, red when the hold fires

use super::channel::Channel;
use super::presets::*;
use crate::pedalkit::abi::Color;
use crate::pedalkit::env::ExpEnvelope;
use crate::pedalkit::helpers::soft_limit;
use crate::pedalkit::pedal::{Param, Pedal};
use crate::pedalkit::pool::Pool;

/// Knob indices.
const DRIP: usize = 0;
const ECHO: usize = 1;
const SPANK: usize = 2;

/// Milliseconds to samples (as the C++ patch rounded them).
const fn ms(v: f32) -> u32 {
    (v * 0.001 * FS) as u32
}

pub struct Patch {
    knobs: [f32; 3],
    drenched: bool,
    last_tap_was_dive: bool,
    kick_led: u32,
    hold_led: u32,
    since_tap: u32,
    channel: Channel,
    /// The wet signal's envelope, driving the LED's brightness step.
    hot: ExpEnvelope,
}

impl Patch {
    /// The flavour changes ranges, ceilings and the echo time, so the knobs
    /// are re-applied after it.
    fn apply_flavour(&mut self) {
        self.channel.set_flavour(if self.drenched { &DRENCHED } else { &CLEAN });
        self.channel.set_drip(self.knobs[DRIP]);
        self.channel.set_echo(self.knobs[ECHO]);
        self.channel.set_spank(self.knobs[SPANK]);
    }

    fn reset_state(&mut self) {
        self.channel.reset();
        self.hot.reset();
        self.kick_led = 0;
        self.hold_led = 0;
        self.since_tap = ms(DOUBLE_TAP_MS);
        self.last_tap_was_dive = false;
    }
}

impl Pedal for Patch {
    const PARAMS: [Param; 3] =
        [Param { name: "Drip", default: 0.65 }, Param { name: "Echo", default: 0.3 }, Param { name: "Spank", default: 0.4 }];
    const POOL: usize = SPRINGS * SPRING_BUFFER + ECHO_BUFFER + DIVE_BUFFER;

    const NEW: Self = Self {
        knobs: [Self::PARAMS[0].default, Self::PARAMS[1].default, Self::PARAMS[2].default],
        drenched: true,
        last_tap_was_dive: false,
        kick_led: 0,
        hold_led: 0,
        since_tap: 0,
        channel: Channel::NEW,
        hot: ExpEnvelope::NEW,
    };

    fn init(&mut self, pool: &mut Pool) {
        self.channel.init(pool);
        self.hot.set(LED_ATTACK_MS, LED_RELEASE_MS, FS);
        self.apply_flavour();
        self.reset_state();
    }

    fn set_knob(&mut self, idx: usize, v: f32) {
        self.knobs[idx] = v;
        match idx {
            DRIP => self.channel.set_drip(v),
            ECHO => self.channel.set_echo(v),
            _ => self.channel.set_spank(v),
        }
    }

    /// One tap is a crash. A second tap inside the double-tap window is the
    /// arm dive on top of the ringing crash; a third starts over.
    fn press(&mut self) {
        if self.since_tap < ms(DOUBLE_TAP_MS) && !self.last_tap_was_dive {
            self.channel.dive.trigger();
            self.last_tap_was_dive = true;
        } else {
            self.channel.tank.kick.trigger(self.knobs[DRIP]);
            self.kick_led = ms(KICK_LED_MS);
            self.last_tap_was_dive = false;
        }
        self.since_tap = 0;
    }

    /// Footswitch hold: Drenched <-> Clean. No reset: the tail keeps ringing
    /// through the change.
    fn hold(&mut self) {
        self.drenched = !self.drenched;
        self.apply_flavour();
        self.hold_led = ms(HOLD_LED_MS);
    }

    #[inline]
    fn tick(&mut self, l: f32, r: f32) -> (f32, f32) {
        let y = soft_limit(self.channel.process((l + r) * 0.5), LIMIT_KNEE).clamp(-1.0, 1.0);
        self.hot.process(self.channel.wet);
        // Timers (the C++ patch stepped these once per block; per sample is
        // the same at every block boundary, where they're read).
        self.kick_led = self.kick_led.saturating_sub(1);
        self.hold_led = self.hold_led.saturating_sub(1);
        self.since_tap = (self.since_tap + 1).min(ms(DOUBLE_TAP_MS));
        (y, y)
    }

    fn led(&self) -> Color {
        if self.hold_led > 0 {
            // The mode switch wins.
            return Color::Red;
        }
        if self.channel.dive.active() {
            // A dive always follows a crash, so it outranks the flash.
            return Color::Magenta;
        }
        if self.kick_led > 0 {
            return Color::DimWhite;
        }
        let ringing = self.hot.env > LED_HOT_THRESH;
        match (self.drenched, ringing) {
            (true, true) => Color::LightBlue,
            (true, false) => Color::Blue,
            (false, true) => Color::DimCyan,
            (false, false) => Color::DimBlue,
        }
    }
}
