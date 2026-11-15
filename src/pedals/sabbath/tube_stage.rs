//! One 12AX7-style stage: gain, a biased waveshaper (even harmonics), and a
//! coupling cap that removes the DC the bias introduces.

use crate::pedalkit::dsp::OnePoleHp;
use crate::pedalkit::helpers::biased;

pub struct TubeStage {
    coupling: OnePoleHp,
    gain: f32,
    bias: f32,
}

impl TubeStage {
    pub const NEW: Self = Self { coupling: OnePoleHp::NEW, gain: 1.0, bias: 0.0 };

    /// Gain, bias, coupling-cap corner (Hz) at sample rate `fs`.
    pub fn init(&mut self, gain: f32, bias: f32, coupling_hz: f32, fs: f32) {
        self.gain = gain;
        self.bias = bias;
        self.coupling.set_at(coupling_hz, fs);
        self.reset();
    }

    pub fn reset(&mut self) {
        self.coupling.reset();
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        self.coupling.process(biased(x * self.gain, self.bias))
    }
}
