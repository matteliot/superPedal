//! The whole outboard unit: the driver tube (DWELL sets how hard it pushes
//! the transducer; more drive saturates it and hits the springs harder,
//! which is where the splash comes from) -> two springs in parallel ->
//! summed wet. The kick goes in at the driver, so it saturates the same way
//! a slammed tank does.

use super::presets::*;
use super::spring::Spring;
use super::tank_kick::TankKick;
use crate::pedalkit::dsp::{Biquad, OnePoleHp};
use crate::pedalkit::helpers::biased;
use crate::pedalkit::pool::Pool;

pub struct SpringTank {
    driver_hpf: OnePoleHp,
    driver_lpf: Biquad,
    dwell: f32,
    springs: [Spring; SPRINGS],
    pub kick: TankKick,
}

impl SpringTank {
    pub const NEW: Self = Self {
        driver_hpf: OnePoleHp::NEW,
        driver_lpf: Biquad::NEW,
        dwell: 1.0,
        springs: [Spring::NEW, Spring::NEW],
        kick: TankKick::NEW,
    };

    pub fn init(&mut self, pool: &mut Pool) {
        self.driver_hpf.set_at(DRIVER_HPF_HZ, FS);
        self.driver_lpf.lowpass_at(DRIVER_LPF_HZ, 0.707, FS);
        for (i, s) in self.springs.iter_mut().enumerate() {
            s.init(SPRING_MS[i], SPRING_CHIRP_HZ[i], pool);
        }
        self.kick.init();
        self.reset();
    }

    pub fn reset(&mut self) {
        self.driver_hpf.reset();
        self.driver_lpf.reset();
        for s in self.springs.iter_mut() {
            s.reset();
        }
        self.kick.reset();
    }

    pub fn set_dwell(&mut self, gain: f32) {
        self.dwell = gain;
    }

    pub fn set_decay(&mut self, loop_gain: f32, damp_hz: f32) {
        for s in self.springs.iter_mut() {
            s.set_decay(loop_gain, damp_hz);
        }
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let k = self.kick.process();
        let drive = self.driver_lpf.process(self.driver_hpf.process(biased((x + k) * self.dwell, DRIVER_BIAS)));
        let mut wet = 0.0;
        for s in self.springs.iter_mut() {
            wet += s.process(drive);
        }
        wet * SPRING_OUT_TRIM
    }
}
