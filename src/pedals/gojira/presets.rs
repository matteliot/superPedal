//! Gojira's settings: every number that shapes the sound.

use crate::pedalkit::clean::{Attack, CleanSettings};
use crate::pedalkit::eq::Eq;
use crate::pedalkit::gate::GateSettings;
use crate::pedalkit::reverb::{FREEVERB_ALLPASSES, FREEVERB_COMBS, ReverbSettings};

/// Preamp stage 1 gain (dB) at knob 0, and the extra at knob 1.
pub const GAIN_MIN_DB: f32 = 22.0;
pub const GAIN_RANGE_DB: f32 = 24.0;
/// Preamp stage 2 gain, dB.
pub const GAIN2_DB: f32 = 14.0;
/// Stage biases: asymmetric clipping (even harmonics, like a tube stage).
pub const BIAS1: f32 = 0.3;
pub const BIAS2: f32 = -0.2;
/// Tight boost: only the content above BOOST_HP is driven into the boost's
/// clipper, so the low end reaches the amp clean and tight.
pub const BOOST_HP: f32 = 720.0;
pub const BOOST_DRIVE: f32 = 4.0;
pub const BOOST_CLIP: f32 = 0.25;
/// Power-amp squash.
pub const POWER_DRIVE: f32 = 1.3;

/// The clean sits this far under the distortion (dB), plus the clean amp's
/// makeup gain so it lands there.
pub const MAGMA_LEVEL_DB: f32 = -5.0;
pub const CLEAN_MAKEUP_DB: f32 = 22.0;
/// Magma: a clean channel with a little sparkle.
pub const CLEAN: CleanSettings = CleanSettings { hp_hz: 90.0, drive: 1.4, attack: Attack::PerSample(0.05), release: 0.05 };
pub const CLEAN_EQ: [Eq; 2] = [Eq::Peak(250.0, 0.8, -2.0), Eq::Peak(2800.0, 0.8, 3.0)];

/// Chorus (clean only): centre delay and depth (samples, 7 ms +- 2 ms), rate
/// (Hz) and wet level at full Space.
pub const CHO_DELAY: f32 = 336.0;
pub const CHO_DEPTH: f32 = 96.0;
pub const CHO_RATE: f32 = 0.7;
pub const CHO_MIX: f32 = 0.6;

/// Echo (clean only): 375 ms (a dotted eighth at 120 bpm), feedback, the
/// low-pass in its loop (Hz) and send at full Space. The send is cut on a
/// switch to Sirius, not the return, so the repeats ring on across it.
pub const ECHO_TIME: f32 = 18_000.0;
pub const ECHO_FB: f32 = 0.38;
pub const ECHO_LP: f32 = 3000.0;
pub const ECHO_SEND: f32 = 0.55;

/// Reverb: wet level at full Space for each voice (the clean is drenched,
/// the distortion only gets a little air), room feedback range, and the room
/// (damping, 22 ms pre-delay).
pub const MAGMA_WET: f32 = 1.0;
pub const SIRIUS_WET: f32 = 0.12;
pub const ROOM_MIN: f32 = 0.78;
pub const ROOM_RANGE: f32 = 0.17;
pub const REVERB: ReverbSettings = ReverbSettings {
    combs: &FREEVERB_COMBS,
    allpasses: &FREEVERB_ALLPASSES,
    spread: 25,
    damp: 0.4,
    in_gain: 0.015,
    out_gain: 3.0,
    predelay: Some(1056.0),
};

/// Whammy: pitch-shifter window (samples, ~37 ms: long enough for drop-C
/// lows), interval (semitones), the swoop's time constant (s), the onset
/// detector (fast envelope this many times the slow one, then locked out
/// this long, s) and the wet crossfade time when armed (s).
pub const WH_WIN: f32 = 1800.0;
pub const WH_SEMIS: f32 = 12.0;
pub const WH_RISE_S: f32 = 0.045;
pub const WH_ONSET: f32 = 1.8;
pub const WH_LOCK_S: f32 = 0.07;
pub const WH_FADE_S: f32 = 0.02;

/// Noise gate on the distortion: fast both ways, so the staccato stops are dead silent.
pub const GATE_OPEN: f32 = 0.0012;
pub const GATE: GateSettings =
    GateSettings { open: GATE_OPEN, close: 0.0005, attack: 0.0005, release: 0.03, fade_in: 0.0005, fade_out: 0.03 };

/// Output trim: unity Level puts the distorted riff around -14 dB RMS.
pub const OUT_TRIM: f32 = 0.32;

/// Closed-back 4x12 with V30-style speakers: low thump, upper-mid bark, a
/// steep roll-off above ~5 kHz.
pub const CAB: [Eq; 6] = [
    Eq::Hp(75.0, 0.8),
    Eq::Peak(110.0, 1.2, 3.0),
    Eq::Peak(400.0, 1.0, -2.0),
    Eq::Peak(2300.0, 1.2, 3.0),
    Eq::Lp(5500.0, 0.8),
    Eq::Lp(6500.0, 0.6),
];
