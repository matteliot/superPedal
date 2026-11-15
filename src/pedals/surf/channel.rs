//! The whole pedal: input HPF -> arm dive -> spank -> tape echo -> spring
//! tank, mixed dry/wet with an equal-power law. The dry path is the spanked,
//! echoed signal, so the springs and the amp both hear the brighter guitar
//! and every echo repeat drips, exactly as a bright single-coil through an
//! echo into a 6G15 would.

use super::arm_dive::ArmDive;
use super::presets::*;
use super::spank::Spank;
use super::spring_tank::SpringTank;
use super::tape_echo::TapeEcho;
use crate::pedalkit::dsp::{Biquad, cos, powf, sin};
use crate::pedalkit::pool::Pool;
use core::f32::consts::PI;

pub struct Channel {
    input_hpf: Biquad,
    pub dive: ArmDive,
    spank: Spank,
    echo: TapeEcho,
    pub tank: SpringTank,
    dry_gain: f32,
    wet_gain: f32,
    /// The last wet sample (the LED reads it).
    pub wet: f32,
    dwell_max: f32,
    wet_max: f32,
    spank_extra_db: f32,
}

impl Channel {
    pub const NEW: Self = Self {
        input_hpf: Biquad::NEW,
        dive: ArmDive::NEW,
        spank: Spank::NEW,
        echo: TapeEcho::NEW,
        tank: SpringTank::NEW,
        dry_gain: 1.0,
        wet_gain: 0.0,
        wet: 0.0,
        dwell_max: DRENCHED.dwell_max,
        wet_max: DRENCHED.wet_max,
        spank_extra_db: DRENCHED.spank_extra_db,
    };

    pub fn init(&mut self, pool: &mut Pool) {
        self.input_hpf.highpass_at(INPUT_HPF_HZ, 0.707, FS);
        // Delay memory: the springs first, then the echo, then the dive line.
        self.tank.init(pool);
        self.echo.init(pool);
        self.dive.init(pool);
        self.spank.init();
        self.reset();
    }

    pub fn reset(&mut self) {
        self.input_hpf.reset();
        self.dive.reset();
        self.spank.reset();
        self.echo.reset();
        self.tank.reset();
        self.wet = 0.0;
    }

    /// The flavour changes decay, damping, dwell range, wet ceiling, spank
    /// shelf and echo time, so the caller re-applies every knob after it.
    pub fn set_flavour(&mut self, f: &Flavour) {
        self.dwell_max = f.dwell_max;
        self.wet_max = f.wet_max;
        self.spank_extra_db = f.spank_extra_db;
        self.tank.set_decay(f.feedback, f.damp_hz);
        self.echo.set_time(f.echo_ms);
    }

    /// One knob for the whole reverb: drive and mix rise together.
    pub fn set_drip(&mut self, knob: f32) {
        self.tank.set_dwell(lerp(DWELL_MIN, self.dwell_max, knob * knob));
        let mix = powf(knob, MIX_CURVE) * MIX_MAX;
        self.dry_gain = cos(mix * PI * 0.5);
        self.wet_gain = sin(mix * PI * 0.5) * self.wet_max;
    }

    pub fn set_echo(&mut self, knob: f32) {
        self.echo.set(knob);
    }

    pub fn set_spank(&mut self, knob: f32) {
        self.spank.set(knob, self.spank_extra_db);
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let dry = self.echo.process(self.spank.process(self.dive.process(self.input_hpf.process(x))));
        self.wet = self.tank.process(dry);
        dry * self.dry_gain + self.wet * self.wet_gain
    }
}
