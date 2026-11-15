//! Harmonics' settings: every number that shapes the sound.

use crate::pedalkit::clean::{Attack, CleanSettings};
use crate::pedalkit::dsp::SR;
use crate::pedalkit::eq::Eq;
use crate::pedalkit::reverb::{FREEVERB_ALLPASSES, FREEVERB_COMBS, ReverbSettings};

/// Pitch tracker: decimation factor, YIN window and lag range (decimated
/// samples: 1.5 kHz down to ~70 Hz), how often it runs (decimated samples),
/// the YIN threshold, the worst dip still accepted when none is under it,
/// the input level it needs, and how fast the tracked pitch glides (s).
pub const DEC: u32 = 4;
pub const DSR: f32 = SR / DEC as f32;
pub const YIN_W: usize = 320;
pub const TAU_MIN: usize = 8;
pub const TAU_MAX: usize = 170;
pub const YIN_HOP: u32 = 64;
pub const YIN_THRESH: f32 = 0.15;
pub const YIN_ACCEPT: f32 = 0.35;
pub const PITCH_MIN_RMS: f32 = 0.002;
pub const GLIDE_S: f32 = 0.008;

/// Mode 2: notch on the fundamental (Q) and a 4th-order high-pass at this
/// multiple of it.
pub const FUND_Q: f32 = 1.0;
pub const M2_HP: f32 = 1.5;
/// Mode 3: resonators (Q) on harmonics 2..=4.
pub const LOW_Q: f32 = 10.0;
/// Mode 4: notches on the 3rd and 4th harmonics (Q) and a 4th-order
/// high-pass at this multiple of the fundamental.
pub const NOTCH4_Q: f32 = 3.0;
pub const M4_HP: f32 = 4.5;
/// Filters stay below this (Hz).
pub const F_MAX: f32 = 16_000.0;
/// Per-mode makeup (dB), so the modes play at about the same level, and the
/// crossfade time on a switch (s).
pub const MAKEUP_DB: [f32; 4] = [0.0, 1.0, 7.3, 2.3];
pub const XFADE_S: f32 = 0.015;

/// Clean amp (a 5 ms attack: a per-sample attack would ride the gain within
/// each cycle, and that modulation puts intermodulation products right on
/// other harmonics (h8 - h5 = h3), undoing the mode filters), its EQ and makeup.
pub const CLEAN: CleanSettings = CleanSettings { hp_hz: 80.0, drive: 1.4, attack: Attack::Time(0.005), release: 0.05 };
pub const CLEAN_EQ: [Eq; 2] = [Eq::Peak(250.0, 0.8, -2.0), Eq::Peak(3000.0, 0.8, 2.5)];
pub const CLEAN_MAKEUP_DB: f32 = 16.0;

/// Small room: feedback and wet level, and the room (damping, 20 ms pre-delay).
pub const ROOM: f32 = 0.82;
pub const WET: f32 = 0.3;
pub const REVERB: ReverbSettings = ReverbSettings {
    combs: &FREEVERB_COMBS,
    allpasses: &FREEVERB_ALLPASSES,
    spread: 25,
    damp: 0.45,
    in_gain: 0.015,
    out_gain: 3.0,
    predelay: Some(960.0),
};

/// Announcements: level at full Voice (peaks are at -1 dBFS in the files),
/// how far the guitar ducks under them and how fast (s).
pub const VOICE_MAX: f32 = 1.0;
pub const VOICE_DEFAULT: f32 = 0.6;
pub const DUCK: f32 = 0.5;
pub const DUCK_S: f32 = 0.03;

/// The spoken mode announcements, 16 kHz G.711 mu-law (see make-prompts.sh).
pub static PROMPTS: [&[u8]; 4] = [
    include_bytes!("prompts/mode1.ulaw"),
    include_bytes!("prompts/mode2.ulaw"),
    include_bytes!("prompts/mode3.ulaw"),
    include_bytes!("prompts/mode4.ulaw"),
];

/// Output trim: unity Level puts a played phrase around -22 dB RMS (single
/// notes and clean chords are spiky, so this leaves their peaks headroom).
pub const OUT_TRIM: f32 = 0.36;

/// A small open-back cab.
pub const CAB: [Eq; 4] = [Eq::Hp(70.0, 0.7), Eq::Peak(150.0, 1.0, 1.5), Eq::Peak(2500.0, 1.2, 2.0), Eq::Lp(9000.0, 0.6)];
