//! The heavy channel: hiss gate -> [2x oversampled: Rangemaster -> three
//! Laney stages -> tone stack] -> power-amp sag -> 4x12.

use super::hiss_gate::HissGate;
use super::oversampler::Oversampler;
use super::presets::*;
use super::rangemaster::Rangemaster;
use super::tone_stack::{ToneStack, lerp};
use super::tube_stage::TubeStage;
use crate::pedalkit::dsp::Biquad;
use crate::pedalkit::env::ExpEnvelope;
use crate::pedalkit::eq::EqChain;

pub struct Heavy {
    input_hpf: Biquad,
    gate: HissGate,
    os: Oversampler,
    rangemaster: Rangemaster,
    lead_mid: Biquad,
    stage: [TubeStage; 3],
    tone: ToneStack,
    sag: ExpEnvelope,
    cab: EqChain<6>,
    drive: f32,
    pub lead: bool,
}

impl Heavy {
    pub const NEW: Self = Self {
        input_hpf: Biquad::NEW,
        gate: HissGate::NEW,
        os: Oversampler::NEW,
        rangemaster: Rangemaster::NEW,
        lead_mid: Biquad::NEW,
        stage: [TubeStage::NEW, TubeStage::NEW, TubeStage::NEW],
        tone: ToneStack::NEW,
        sag: ExpEnvelope::NEW,
        cab: EqChain::NEW,
        drive: 1.0,
        lead: false,
    };

    pub fn init(&mut self) {
        self.input_hpf.highpass_at(INPUT_HPF_HZ, 0.707, FS);
        self.gate.init();
        self.os.init();
        self.rangemaster.init();
        self.lead_mid.peak_at(LEAD_MID_HZ, 0.9, LEAD_MID_DB, FS_OS);
        for (i, s) in self.stage.iter_mut().enumerate() {
            s.init(STAGE_GAIN[i], STAGE_BIAS[i], COUPLING_HZ[i], FS_OS);
        }
        self.tone.init();
        self.sag.set(SAG_ATTACK_MS, SAG_RELEASE_MS, FS);
        self.cab.init(&CAB);
        self.reset();
    }

    pub fn reset(&mut self) {
        self.input_hpf.reset();
        self.gate.reset();
        self.os.reset();
        self.rangemaster.reset();
        self.lead_mid.reset();
        for s in self.stage.iter_mut() {
            s.reset();
        }
        self.tone.reset();
        self.sag.reset();
        self.cab.reset();
    }

    /// GAIN knob -> Rangemaster gain and preamp drive (audio taper); LEAD
    /// multiplies the Rangemaster gain.
    pub fn set_gain(&mut self, knob: f32) {
        let g = knob * knob;
        self.rangemaster.gain = lerp(RM_GAIN_MIN, RM_GAIN_MAX, g) * if self.lead { LEAD_GAIN } else { 1.0 };
        self.drive = lerp(DRIVE_MIN, DRIVE_MAX, g);
    }

    pub fn set_tone(&mut self, knob: f32) {
        self.tone.set(knob);
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let x = self.gate.process(self.input_hpf.process(x));
        let (rm, lead, lead_mid, drive, stage, tone) =
            (&mut self.rangemaster, self.lead, &mut self.lead_mid, self.drive, &mut self.stage, &mut self.tone);
        // One sample of the nonlinear chain at 96 kHz.
        let x = self.os.process(x, |v| {
            let mut v = rm.process(v);
            if lead {
                v = lead_mid.process(v);
            }
            v *= drive;
            for s in stage.iter_mut() {
                v = s.process(v);
            }
            tone.process(v)
        });
        // Power-amp sag: hard hits pull the rails down (the bloom).
        let x = x / (1.0 + SAG_AMOUNT * self.sag.process(x) * 4.0);
        self.cab.process(x)
    }
}
