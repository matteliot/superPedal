//! Led Zeppelin's settings: every number that shapes the sound.

use crate::pedalkit::eq::Eq;
use crate::pedalkit::gate::GateSettings;
use crate::pedalkit::reverb::ReverbSettings;

/// Les Paul volume pot: level (dB) at Gain 0 (it's 0 dB at Gain 1), and its treble loss: the low-pass corner at Gain 0 and the extra at Gain 1 (Hz).
pub const POT_FLOOR_DB: f32 = -30.0;
pub const POT_LP_MIN: f32 = 6500.0;
pub const POT_LP_RANGE: f32 = 11000.0;
/// Below the default Gain, this much makeup (dB per unit of knob below it) keeps a backed-off guitar from dropping out of the mix while still cleaning up.
pub const GAIN_DEFAULT: f32 = 0.8;
pub const GAIN_MAKEUP_DB: f32 = 12.0;

/// Echoplex EP-3 preamp: gain, bias (asymmetry) and the top-end lift.
pub const EP_GAIN: f32 = 1.6;
pub const EP_BIAS: f32 = 0.15;

/// Tone Bender MkII: fuzz gain (dB), the second stage's bias, and its output level into the amp.
pub const TB_GAIN_DB: f32 = 32.0;
pub const TB_BIAS: f32 = 0.3;
pub const TB_OUT: f32 = 0.22;

/// Plexi: the two cascaded preamp triodes' gains and bias, the power amp's drive, and its sag: how much the supply droops under a loud note, above the knee.
pub const PRE1_GAIN: f32 = 8.0;
pub const PRE2_GAIN: f32 = 2.5;
pub const TRIODE_BIAS: f32 = 0.1;
pub const POWER_GAIN: f32 = 2.2;
pub const SAG: f32 = 1.0;
pub const SAG_KNEE: f32 = 0.5;

/// Echoplex tape echo: delay (s), feedback, wow (0.6 Hz) and flutter (7 Hz) depth as fractions of the delay, and its level.
pub const ECHO_S: f32 = 0.32;
pub const ECHO_FB: f32 = 0.38;
pub const WOW_HZ: f32 = 0.6;
pub const WOW_DEPTH: f32 = 0.0025;
pub const FLUTTER_HZ: f32 = 7.0;
pub const FLUTTER_DEPTH: f32 = 0.0006;
pub const ECHO_MIX: f32 = 0.5;
pub const ECHO_PAN: (f32, f32) = (0.45, 1.0);

/// Room mics: the late room's comb feedback (it's a short Freeverb-style tail, about 0.55x Freeverb's lengths at 48 kHz; the right channel is spread by 23), early-reflection taps (ms, gain), pre-delay of the late room (ms), compressor amount, and the level at Room 1.
pub const ROOM_FEEDBACK: f32 = 0.76;
pub const ROOM_TAIL: ReverbSettings = ReverbSettings {
    combs: &[677, 719, 773, 821],
    allpasses: &[347, 263],
    spread: 23,
    damp: 0.45,
    in_gain: 0.25,
    out_gain: 1.0,
    predelay: None,
};
pub const ER_TAPS: [(f32, f32); 6] = [(7.3, 0.55), (11.9, 0.5), (17.1, 0.42), (23.6, 0.36), (31.2, 0.3), (41.5, 0.24)];
pub const ROOM_PREDELAY_MS: f32 = 18.0;
pub const ROOM_COMP: f32 = 6.0;
pub const ROOM_GAIN: f32 = 4.5;
/// How much of the close mic the Room knob takes away at 1 (the room mics take over).
pub const ROOM_DRY_DUCK: f32 = 0.45;

/// Noise gate on the input envelope.
pub const GATE: GateSettings =
    GateSettings { open: 0.0012, close: 0.0006, attack: 0.001, release: 0.08, fade_in: 0.002, fade_out: 0.06 };

/// Output trim: unity Level puts the Plexi riff around -12 dB RMS.
pub const OUT_TRIM: f32 = 0.42;

/// 4x12 Greenbacks: closed-back resonance, upper-mid bark, steep roll-off above 5 kHz.
pub const CAB: [Eq; 6] = [
    Eq::Hp(75.0, 0.9),
    Eq::Peak(110.0, 1.2, 3.0),
    Eq::Peak(2200.0, 1.5, 4.0),
    Eq::Peak(4200.0, 2.0, -3.0),
    Eq::Lp(5200.0, 0.8),
    Eq::Lp(7500.0, 0.6),
];
