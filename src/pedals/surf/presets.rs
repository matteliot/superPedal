//! Surf's settings (from the C++ patch's Tuning.h): every number that shapes
//! the sound.

pub const FS: f32 = 48_000.0;

// Input
/// Rumble only; the Twin handles the rest.
pub const INPUT_HPF_HZ: f32 = 40.0;

// Spank (Jaguar/Jazzmaster brightness + pick attack): the knob scales all three.
pub const SPANK_SHELF_HZ: f32 = 2200.0;
/// At SPANK=1.
pub const SPANK_SHELF_DB: f32 = 7.0;
pub const SPANK_PEAK_HZ: f32 = 3800.0;
pub const SPANK_PEAK_DB: f32 = 3.0;
/// Transient boost amount at SPANK=1 (gain = 1 + amount * attack).
pub const SPANK_TRANSIENT: f32 = 1.2;
pub const SPANK_FAST_MS: f32 = 0.5;
pub const SPANK_FAST_REL_MS: f32 = 25.0;
pub const SPANK_SLOW_MS: f32 = 12.0;
pub const SPANK_SLOW_REL_MS: f32 = 180.0;

// DRIP knob: one knob drives the tank harder and mixes more of it in at the
// same time, the way a surf player sets a 6G15 (dwell and mix go up together).
/// Driver gain at DRIP=0.
pub const DWELL_MIN: f32 = 0.4;
/// Mix position = DRIP^curve: the wet comes in early, then keeps growing.
pub const MIX_CURVE: f32 = 0.6;
/// Mix position at DRIP=1 as a fraction of the quarter turn: dry never fully
/// disappears (0.8 -> dry x0.31, wet x0.95).
pub const MIX_MAX: f32 = 0.8;

// Spring driver (the 6G15's 12AT7 driving the tank input transducer)
/// Tube asymmetry.
pub const DRIVER_BIAS: f32 = 0.1;
/// The transducer can't move air below this.
pub const DRIVER_HPF_HZ: f32 = 130.0;
/// 2nd-order: above the springs' transition frequency the model passes
/// energy fast, and that click would mask the chirp.
pub const DRIVER_LPF_HZ: f32 = 4000.0;

// Springs: two of different lengths, each a delay loop with a dispersive
// allpass cascade (the chirp that reads as "drip").
pub const SPRINGS: usize = 2;
pub const SPRING_MS: [f32; SPRINGS] = [31.0, 37.5];
/// Transition frequency: group delay peaks here, so highs up to it lag the
/// lows (upward chirp).
pub const SPRING_CHIRP_HZ: [f32; SPRINGS] = [2800.0, 3300.0];
/// Allpass pole radius: higher = a sharper, taller delay hump around the
/// transition frequency.
pub const SPRING_CHIRP_R: f32 = 0.84;
/// Per spring; total chirp spread ~2.5-3 ms at these settings, more stages = more boing.
pub const ALLPASS_STAGES: usize = 20;
/// In the loop: springs don't carry bass.
pub const SPRING_HPF_HZ: f32 = 80.0;
/// Wet level trim after summing the springs.
pub const SPRING_OUT_TRIM: f32 = 0.55;
pub const DENORMAL_FLOOR: f32 = 1e-20;

// Tape echo (ECHO knob): a slapback in front of the tank, so every repeat
// drips too. The time is set by the flavour; the knob sets level and how many repeats.
/// Repeat level relative to dry at ECHO=1.
pub const ECHO_LEVEL_MAX: f32 = 0.8;
/// Loop gain at ECHO=1: a clear slap plus a couple of fading repeats; scales
/// with knob^2 so low settings are one slap.
pub const ECHO_FEEDBACK_MAX: f32 = 0.5;
/// Tape / head bandwidth, in the loop so each repeat is darker.
pub const ECHO_LPF_HZ: f32 = 4500.0;
pub const ECHO_HPF_HZ: f32 = 120.0;
/// Head bump.
pub const ECHO_BUMP_HZ: f32 = 200.0;
pub const ECHO_BUMP_DB: f32 = 2.0;
/// Capstan wow and flutter: rate (Hz), +- depth (ms).
pub const ECHO_WOW_HZ: f32 = 0.7;
pub const ECHO_WOW_MS: f32 = 0.35;
pub const ECHO_FLUTTER_HZ: f32 = 6.3;
pub const ECHO_FLUTTER_MS: f32 = 0.04;

// Vibrato-arm dive (double-tap): a Jazzmaster arm pushed down and let go:
// pitch falls, then springs back with a damped wobble.
/// Depth of the push (semitones), time to reach the bottom (ms, raised cosine).
pub const DIVE_SEMIS: f32 = 2.5;
pub const DIVE_DOWN_MS: f32 = 180.0;
/// The arm's spring-back oscillation (Hz) and its decay (ms).
pub const DIVE_WOBBLE_HZ: f32 = 4.0;
pub const DIVE_WOBBLE_TAU_MS: f32 = 120.0;
/// How long the wobble is followed before the read point is faded home (ms).
pub const DIVE_WOBBLE_MS: f32 = 600.0;
/// Crossfade from the accumulated delay back to the base delay (ms): the
/// residual latency the pitch drop left behind.
pub const DIVE_RETURN_MS: f32 = 60.0;
/// Idle delay: enough for a fractional read.
pub const DIVE_BASE_SAMPLES: f32 = 2.0;
/// A second tap inside this window (ms) is a dive, not another crash.
pub const DOUBLE_TAP_MS: f32 = 400.0;

/// Flavours (footswitch hold): Drenched = Dick Dale's cranked 6G15, Clean =
/// Ventures / Shadows restraint (shorter tank, longer echo).
pub struct Flavour {
    /// Driver gain at DRIP=1.
    pub dwell_max: f32,
    /// Loop gain: the decay length.
    pub feedback: f32,
    /// Loop low-pass (Hz): tail brightness / splash.
    pub damp_hz: f32,
    /// Wet level at full mix.
    pub wet_max: f32,
    /// Extra shelf on top of the Spank knob (dB).
    pub spank_extra_db: f32,
    /// Slapback time (ms).
    pub echo_ms: f32,
}

pub const DRENCHED: Flavour =
    Flavour { dwell_max: 9.0, feedback: 0.955, damp_hz: 4000.0, wet_max: 1.0, spank_extra_db: 2.0, echo_ms: 140.0 };
pub const CLEAN: Flavour =
    Flavour { dwell_max: 4.0, feedback: 0.88, damp_hz: 2700.0, wet_max: 0.65, spank_extra_db: 0.0, echo_ms: 230.0 };

// Tank kick (footswitch tap): a noise splash plus a low thud slammed into both springs.
/// Burst decay (ms); thud frequency (Hz) and decay (ms); level, scaled by (0.5 + DRIP).
pub const KICK_NOISE_MS: f32 = 22.0;
pub const KICK_THUD_HZ: f32 = 58.0;
pub const KICK_THUD_MS: f32 = 110.0;
pub const KICK_LEVEL: f32 = 2.5;
/// The LED's white flash length (ms).
pub const KICK_LED_MS: f32 = 250.0;

// Output
pub const LIMIT_KNEE: f32 = 0.8;

// LED
/// Wet-signal envelope that counts as "springs ringing hard", and its attack / release (ms).
pub const LED_HOT_THRESH: f32 = 0.07;
pub const LED_ATTACK_MS: f32 = 5.0;
pub const LED_RELEASE_MS: f32 = 150.0;
/// Red after the hold fires (the flavour switch), ms.
pub const HOLD_LED_MS: f32 = 700.0;

// Delay memory (floats): per spring, the echo (> 300 ms) and the dive (> 80 ms).
pub const SPRING_BUFFER: usize = 2048;
pub const ECHO_BUFFER: usize = 16384;
pub const DIVE_BUFFER: usize = 4096;

/// Recursive filters ringing down forever end in subnormal floats; snap those to zero.
#[inline]
pub fn kill_denormal(x: f32) -> f32 {
    if x.abs() < DENORMAL_FLOOR { 0.0 } else { x }
}

#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
