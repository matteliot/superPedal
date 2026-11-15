//! Soundgarden's settings: every number that shapes the sound.

use crate::pedalkit::eq::Eq;
use crate::pedalkit::feedback::FeedbackSettings;
use crate::pedalkit::gate::GateSettings;

/// Modes, cycled by footswitch press.
pub const HEAVY: u8 = 0;
pub const BLACK_HOLE: u8 = 1;
pub const BLACK_HOLE_FAST: u8 = 2;
pub const MODES: u8 = 3;

/// Fuzz gain (dB) at knob 0 and the extra at knob 1, split across two stages.
pub const FUZZ_MIN_DB: f32 = 6.0;
pub const FUZZ_RANGE_DB: f32 = 44.0;
/// Black Hole modes clean up the fuzz this much (dB) for the psychedelic parts.
pub const BLACK_HOLE_FUZZ_CUT_DB: f32 = 14.0;
/// Asymmetric bias of the first clipper (even harmonics, "sputter").
pub const ASYM_BIAS: f32 = 0.3;

/// Noise gate on the input envelope.
pub const GATE: GateSettings =
    GateSettings { open: 0.0012, close: 0.0006, attack: 0.001, release: 0.08, fade_in: 0.002, fade_out: 0.06 };
/// Black Hole modes run 14 dB less fuzz (so 14 dB less hiss), and their notes
/// are left to ring: the gate sits this much lower (dB) and fades out slowly,
/// so it doesn't chop the tails off.
pub const BLACK_HOLE_GATE_DB: f32 = -18.0;
pub const BLACK_HOLE_GATE_FADE_S: f32 = 0.3;

/// Feedback loop: delay (samples, ~ distance amp -> guitar), band-pass centre
/// and Q (where the howl blooms), loop gain at full bloom; it blooms over ~3 s
/// while a note sustains (~95% at 3 s) and dies in 80 ms when muted.
pub const FEEDBACK: FeedbackSettings =
    FeedbackSettings { delay: 331.0, freq: 1250.0, q: 3.0, gain: 1.8, bloom: 3.0 / 3.0, die: 0.08 };

/// Leslie: rotor speeds (Hz), speed-change inertia (s), crossover, Doppler
/// depth / base delay (samples), amplitude modulation depth.
pub const HORN_SLOW: f32 = 0.83;
pub const HORN_FAST: f32 = 6.7;
pub const DRUM_SLOW: f32 = 0.67;
pub const DRUM_FAST: f32 = 5.8;
pub const HORN_INERTIA_S: f32 = 0.6;
pub const DRUM_INERTIA_S: f32 = 3.0;
pub const XOVER_HZ: f32 = 800.0;
pub const HORN_DOPPLER: f32 = 40.0;
pub const DRUM_DOPPLER: f32 = 12.0;
pub const ROT_BASE: f32 = 4.0;
pub const HORN_AM: f32 = 0.55;
pub const DRUM_AM: f32 = 0.3;
/// Right mic angle relative to the left mic (radians).
pub const MIC_R_ANGLE: f32 = 0.6 * core::f32::consts::PI;

/// Output trim: unity Level puts the Heavy riff around -14 dB RMS.
pub const OUT_TRIM: f32 = 0.27;
/// Black Hole modes run less fuzz, so they get this much output makeup (dB).
pub const BLACK_HOLE_MAKEUP_DB: f32 = 9.0;

/// 4x12 with greenback/V30-style voicing: thump, presence, steep top roll-off.
pub const CAB: [Eq; 6] = [
    Eq::Hp(75.0, 0.707),
    Eq::Peak(115.0, 1.4, 4.0),
    Eq::Peak(2400.0, 1.8, 3.0),
    Eq::Peak(5500.0, 2.0, -4.0),
    Eq::Lp(4600.0, 0.8),
    Eq::Lp(6500.0, 0.6),
];
