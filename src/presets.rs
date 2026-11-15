//! superPedal's settings.

use crate::rack::POSITIONS;

/// Each position's output trim (dB), so every pedal plays at the same
/// loudness at its own default settings and the amp sets the volume
/// (measured by `CALIBRATE=1 cargo sim`; "no pedal" is the dry guitar,
/// untouched). Order: as `rack::NAMES`.
pub const TRIM_DB: [f32; POSITIONS] = [0.0, -3.8, 1.4, -7.5, -3.7, -5.0, -3.4, -5.8, 0.0, -3.5, -5.6, 1.2];

/// The loudness every pedal is matched to (dB RMS on the calibration phrase).
pub const TARGET_DB: f32 = -18.0;

/// Announcements: level, how far the music ducks under them and how fast (s).
pub const VOICE_LEVEL: f32 = 0.36;
pub const DUCK: f32 = 0.5;
pub const DUCK_S: f32 = 0.03;

/// A pedal change: fade-out / fade-in time constant (s), and how much of
/// the new pedal's delay memory is cleared per audio block (floats; the
/// output is silent meanwhile).
pub const FADE_S: f32 = 0.004;
pub const CLEAR_PER_BLOCK: usize = 16_384;

/// Output limiter knee: only peaks above it are touched.
pub const LIMIT_KNEE: f32 = 0.95;
