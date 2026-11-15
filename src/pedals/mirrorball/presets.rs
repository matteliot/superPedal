//! Mirrorball's settings: every number that shapes the sound.

use crate::pedalkit::eq::Eq;
use crate::pedalkit::reverb::{FREEVERB_ALLPASSES, FREEVERB_COMBS, ReverbSettings};

/// Distorted-voice preamp stage 1 gain (dB) at knob 0, and the extra at knob 1.
/// Measured on a held dyad (`cargo sim`): ~4% distortion at 0, ~20% (the
/// Downtown crunch) at the default, ~55% (saturated, Ocean) at 1.
pub const DRIVE_MIN_DB: f32 = 0.0;
pub const DRIVE_RANGE_DB: f32 = 38.0;
pub const DRIVE_DEFAULT: f32 = 0.4;
/// Distorted-voice preamp stage 2 gain, dB.
pub const DRIVE2_DB: f32 = 8.0;
/// Stage biases: asymmetric clipping (even harmonics, like a tube stage).
pub const BIAS1: f32 = 0.25;
pub const BIAS2: f32 = -0.15;
/// Power amp: drive into the 6V6s and their bias.
pub const POWER_DRIVE: f32 = 2.0;
pub const POWER_BIAS: f32 = 0.1;
/// Rectifier sag: how far a loud signal pulls the power amp's gain down, and
/// the supply's attack / recovery (s). The recovery is what makes a held
/// chord swell back up after the pick hits.
pub const SAG: f32 = 1.5;
pub const SAG_ATT_S: f32 = 0.01;
pub const SAG_REL_S: f32 = 0.15;

/// Peace: the same amp at a low volume. Preamp gain (dB), and the voice's
/// level under Downtown plus its makeup gain so it lands there.
pub const PEACE_DRIVE_DB: f32 = 10.0;
pub const PEACE_LEVEL_DB: f32 = -5.0;
pub const PEACE_MAKEUP_DB: f32 = 7.5;

/// Feedback (Downtown only). Real feedback locks onto a harmonic of the string,
/// because the string is the resonator; a free-running loop in the pedal
/// would whistle at its own pitch instead. So this is a sustainer: a held
/// note blooms once nothing has been picked for FB_WAIT seconds while the
/// string is still ringing above FB_HOLD. The note is then held at the level
/// it had (up to FB_MAX_DB of makeup as the string decays), its body pulled
/// down to FB_DRY and the partials inside a slowly drifting band (centre,
/// Q, octaves of drift, drift rate) pushed up by FB_HARM, so the amp locks
/// onto a harmonic of whatever is held and wanders between harmonics. It
/// holds until the next pick or until the strings are muted below FB_MUTE.
/// Bloom / die times (s).
pub const FB_WAIT: f32 = 0.35;
pub const FB_HOLD: f32 = 0.002;
pub const FB_MUTE: f32 = 0.0002;
pub const FB_MAX_DB: f32 = 30.0;
pub const FB_DRY: f32 = 0.5;
pub const FB_HARM: f32 = 1.5;
pub const FB_FREQ: f32 = 1100.0;
pub const FB_Q: f32 = 4.0;
pub const FB_WANDER: f32 = 0.4;
pub const FB_RATE: f32 = 0.07;
pub const FB_BLOOM_S: f32 = 0.6;
pub const FB_DIE_S: f32 = 0.08;
/// Pick onset detector: fast envelope this many times the slow one, above
/// this level.
pub const ONSET: f32 = 1.8;
pub const ONSET_MIN: f32 = 0.003;

/// Bigsby: the base delay and the swing (samples; ~ +-20 cents at the
/// centre rate), the shake rate (Hz), how much it drifts (fraction, at
/// WOB_DRIFT_HZ), how much the depth varies (at WOB_SWAY_HZ) and how fast
/// the hand gets on / off the bar (s).
pub const WOB_BASE: f32 = 2.0;
pub const WOB_DEPTH: f32 = 56.0;
pub const WOB_RATE: f32 = 4.8;
pub const WOB_DRIFT: f32 = 0.25;
pub const WOB_DRIFT_HZ: f32 = 0.23;
pub const WOB_SWAY: f32 = 0.3;
pub const WOB_SWAY_HZ: f32 = 0.37;
pub const WOB_HAND_S: f32 = 0.15;

/// Room: wet level at full Room for each voice, feedback range, and the room
/// itself (damping, 12 ms pre-delay): a live room, not a hall.
pub const PEACE_WET: f32 = 0.7;
pub const OCEAN_WET: f32 = 0.5;
pub const ROOM_MIN: f32 = 0.70;
pub const ROOM_RANGE: f32 = 0.2;
pub const REVERB: ReverbSettings = ReverbSettings {
    combs: &FREEVERB_COMBS,
    allpasses: &FREEVERB_ALLPASSES,
    spread: 25,
    damp: 0.45,
    in_gain: 0.015,
    out_gain: 3.0,
    predelay: Some(576.0),
};

/// Output trim: unity Level puts Downtown's chords around -14 dB RMS.
pub const OUT_TRIM: f32 = 0.42;

/// Open-back 1x12, Jensen-style: lean bottom, honky upper mids, early roll-off.
pub const CAB: [Eq; 6] = [
    Eq::Hp(70.0, 0.7),
    Eq::Peak(200.0, 1.0, 2.0),
    Eq::Peak(1200.0, 1.0, 2.0),
    Eq::Peak(2800.0, 1.5, 3.0),
    Eq::Lp(5000.0, 0.7),
    Eq::Lp(7500.0, 0.6),
];
