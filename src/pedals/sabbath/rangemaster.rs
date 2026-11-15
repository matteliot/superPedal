//! Dallas Rangemaster: input cap corner, big gain, germanium asymmetric clip,
//! HF roll-off. Runs in the oversampled domain.

use super::presets::*;
use crate::pedalkit::dsp::{OnePole, OnePoleHp, tanh};

pub struct Rangemaster {
    hpf: OnePoleHp,
    lpf: OnePole,
    pub gain: f32,
}

impl Rangemaster {
    pub const NEW: Self = Self { hpf: OnePoleHp::NEW, lpf: OnePole::NEW, gain: 1.0 };

    pub fn init(&mut self) {
        self.hpf.set_at(RM_HPF_HZ, FS_OS);
        self.lpf.set_at(RM_LPF_HZ, FS_OS);
        self.reset();
    }

    pub fn reset(&mut self) {
        self.hpf.reset();
        self.lpf.reset();
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        self.lpf.process(germanium(self.hpf.process(x) * self.gain))
    }
}

/// Germanium transistor: soft knee, the positive rail clips early.
#[inline]
fn germanium(x: f32) -> f32 {
    let y = tanh(x);
    if y > RM_POS_CLIP { RM_POS_CLIP + (y - RM_POS_CLIP) * 0.25 } else { y }
}
