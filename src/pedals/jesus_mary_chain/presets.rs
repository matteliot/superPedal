//! The Jesus and Mary Chain's settings: every number that shapes the sound.

use crate::pedalkit::clean::{Attack, CleanSettings};
use crate::pedalkit::eq::Eq;
use crate::pedalkit::feedback::FeedbackSettings;
use crate::pedalkit::gate::GateSettings;
use crate::pedalkit::reverb::{FREEVERB_ALLPASSES, FREEVERB_COMBS, ReverbSettings};

/// Fuzz gain (dB) at knob 0, and the extra at knob 1.
pub const FUZZ_MIN_DB: f32 = 20.0;
pub const FUZZ_RANGE_DB: f32 = 30.0;
/// Second (hard clipper) stage gain, dB: fuzz into fuzz.
pub const FUZZ2_DB: f32 = 16.0;
/// Rectified (octave-up) content mixed into the first fuzz stage.
pub const OCTAVE: f32 = 0.45;

/// The clean sits this far under the fuzz (dB), plus the clean amp's makeup
/// gain so it lands there.
pub const HONEY_LEVEL_DB: f32 = -5.0;
pub const CLEAN_MAKEUP_DB: f32 = 23.0;
/// Honey: bright, jangly clean (a hollow-body through a clean combo).
pub const CLEAN: CleanSettings = CleanSettings { hp_hz: 100.0, drive: 1.6, attack: Attack::PerSample(0.05), release: 0.05 };
pub const CLEAN_EQ: [Eq; 2] = [Eq::Peak(250.0, 0.8, -2.5), Eq::Peak(3600.0, 0.8, 5.0)];

/// Reverb: wet level at full Reverb knob for each voice (the clean is
/// drenched, the fuzz a little drier so it stays a wall rather than mush),
/// room feedback range, and the room itself (damping, 22 ms pre-delay).
pub const HONEY_WET: f32 = 1.15;
pub const PSYCHO_WET: f32 = 0.8;
pub const ROOM_MIN: f32 = 0.80;
pub const ROOM_RANGE: f32 = 0.165;
pub const REVERB: ReverbSettings = ReverbSettings {
    combs: &FREEVERB_COMBS,
    allpasses: &FREEVERB_ALLPASSES,
    spread: 25,
    damp: 0.38,
    in_gain: 0.015,
    out_gain: 3.0,
    predelay: Some(1056.0),
};

/// Feedback squall: loop delay (samples, ~ distance amp -> guitar), band-pass
/// Q, the centre it wanders around (Hz), loop gain at full bloom, and bloom
/// (~95% after 1.2 s) / die times (s). Then how far (octaves) the centre
/// wanders, the wander rates (Hz), and how far the string's own sound is
/// pushed down once the squall has taken it over.
pub const FEEDBACK: FeedbackSettings =
    FeedbackSettings { delay: 263.0, freq: 2300.0, q: 7.0, gain: 1.5, bloom: 1.2 / 3.0, die: 0.25 };
pub const FB_FREQ: f32 = 2300.0;
pub const FB_WANDER: f32 = 0.65;
pub const FB_RATE_A: f32 = 0.11;
pub const FB_RATE_B: f32 = 0.29;
pub const FB_DUCK: f32 = 0.6;

/// Noise gate on the fuzz; it closes slowly so tails and squall die
/// naturally rather than chop.
pub const GATE: GateSettings =
    GateSettings { open: 0.0010, close: 0.0004, attack: 0.001, release: 0.08, fade_in: 0.002, fade_out: 0.12 };

/// Output trim: unity Level puts the fuzz strums around -14 dB RMS.
pub const OUT_TRIM: f32 = 0.16;

/// Small open-back combo: less bottom and more top than a 4x12.
pub const CAB: [Eq; 6] = [
    Eq::Hp(85.0, 0.707),
    Eq::Peak(120.0, 1.0, 2.0),
    Eq::Peak(2500.0, 1.2, 2.0),
    Eq::Peak(5000.0, 2.0, -2.0),
    Eq::Lp(7000.0, 0.7),
    Eq::Lp(10_000.0, 0.6),
];
