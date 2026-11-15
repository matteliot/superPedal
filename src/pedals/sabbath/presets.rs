//! Sabbath's settings (from the C++ patch's Tuning.h): every number that
//! shapes the sound. The clipping stages run at 2x oversampling (96 kHz).

pub const FS: f32 = 48_000.0;
pub const FS_OS: f32 = FS * 2.0;

// Input
/// Sub-mud; Iommi is tuned to C#, keep it low.
pub const INPUT_HPF_HZ: f32 = 55.0;
/// Gentle hiss gate for high-gain settings: threshold and hysteresis (dB),
/// and the per-sample gain ramp when opening and closing.
pub const GATE_THRESH_DB: f32 = -62.0;
pub const GATE_HYST_DB: f32 = 6.0;
pub const GATE_ATTACK: f32 = 0.05;
pub const GATE_RELEASE: f32 = 0.002;

/// Oversampling image / anti-alias corner in the 96 kHz domain.
pub const OS_FILTER_HZ: f32 = 19_000.0;

// Rangemaster (germanium OC44 treble booster, Iommi's was modded toward full-range)
/// Input cap corner: 260 Hz = mid-tilt, not pure treble.
pub const RM_HPF_HZ: f32 = 260.0;
/// Linear gain at GAIN=0 and GAIN=1 (~23 dB).
pub const RM_GAIN_MIN: f32 = 3.0;
pub const RM_GAIN_MAX: f32 = 14.0;
/// Germanium asymmetry: the positive half clips earlier.
pub const RM_POS_CLIP: f32 = 0.55;
/// Germanium HF roll-off.
pub const RM_LPF_HZ: f32 = 9000.0;

// Laney preamp: three 12AX7-style stages
pub const STAGE_GAIN: [f32; 3] = [4.0, 3.5, 2.5];
/// More asymmetry deeper in.
pub const STAGE_BIAS: [f32; 3] = [0.12, 0.18, 0.25];
/// Coupling caps tighten the low end.
pub const COUPLING_HZ: [f32; 3] = [18.0, 35.0, 60.0];
/// Preamp input trim at GAIN=0 and GAIN=1.
pub const DRIVE_MIN: f32 = 0.35;
pub const DRIVE_MAX: f32 = 1.6;

// Lead boost
/// +6 dB into the Rangemaster, and a mid push.
pub const LEAD_GAIN: f32 = 2.0;
pub const LEAD_MID_HZ: f32 = 850.0;
pub const LEAD_MID_DB: f32 = 3.5;

// Tone stack (after stage 3, before the power amp). TONE morphs between the dark and bright settings.
pub const LOW_SHELF_HZ: f32 = 180.0;
pub const LOW_DB_DARK: f32 = 4.0;
pub const LOW_DB_BRIGHT: f32 = -3.0;
pub const MID_HZ: f32 = 550.0;
/// The bright end scoops the mids.
pub const MID_DB_DARK: f32 = 2.0;
pub const MID_DB_BRIGHT: f32 = -4.0;
pub const HIGH_SHELF_HZ: f32 = 2200.0;
pub const HIGH_DB_DARK: f32 = -9.0;
pub const HIGH_DB_BRIGHT: f32 = 5.0;

// Power amp sag
pub const SAG_AMOUNT: f32 = 0.35;
pub const SAG_ATTACK_MS: f32 = 4.0;
pub const SAG_RELEASE_MS: f32 = 120.0;

// Cab sim: Celestion-loaded 4x12, closed back
pub const CAB: [crate::pedalkit::eq::Eq; 6] = {
    use crate::pedalkit::eq::Eq;
    [
        Eq::Hp(85.0, 0.8),
        Eq::Peak(115.0, 1.2, 3.0),
        Eq::Peak(420.0, 0.8, -2.5),
        Eq::Peak(2300.0, 1.1, 4.0),
        Eq::Lp(4800.0, 0.9),
        Eq::Lp(6200.0, 0.6),
    ]
};

// Output
/// Post-cab trim so LEVEL ~0.7 sits just under the limiter; LEVEL=1 is +4 dB;
/// the soft limiter starts at the knee, with its ceiling at 1.0.
pub const OUT_TRIM: f32 = 0.5;
pub const LEVEL_MAX: f32 = 1.6;
pub const LIMIT_KNEE: f32 = 0.8;

// Planet Caravan (a Leslie rotary on a clean-ish channel)
/// Gentle single-stage warmth.
pub const PC_GAIN: f32 = 1.4;
pub const PC_BIAS: f32 = 0.08;
pub const PC_COUPLING_HZ: f32 = 25.0;
/// Rotor rate: chorale to tremolo.
pub const PC_RATE_MIN_HZ: f32 = 0.6;
pub const PC_RATE_MAX_HZ: f32 = 7.0;
/// Doppler delay centre and maximum swing (ms), amplitude modulation depth.
pub const PC_DELAY_MS: f32 = 4.0;
pub const PC_DEPTH_MAX_MS: f32 = 2.5;
pub const PC_AM_DEPTH_MAX: f32 = 0.6;
/// Horn / drum split.
pub const PC_CROSSOVER_HZ: f32 = 800.0;
/// The drum spins slower than the horn, as in a real Leslie, with a phase
/// offset so horn and drum never line up.
pub const PC_DRUM_RATIO: f32 = 0.66;
pub const PC_DRUM_OFFSET: f32 = 0.37;
/// Level-match the clean channel to the heavy one.
pub const PC_OUT_TRIM: f32 = 1.6;

// LED
/// Clip-stage envelope that counts as "hot", and its attack / release (ms).
pub const LED_HOT_THRESH: f32 = 0.45;
pub const LED_ATTACK_MS: f32 = 2.0;
pub const LED_RELEASE_MS: f32 = 80.0;
