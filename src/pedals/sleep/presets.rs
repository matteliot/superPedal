//! Sleep's settings for the pedalkit blocks: the two stacks, the Muff, the
//! parked wah and the 4x12.

use crate::pedalkit::amp::voicing::Voicing;
use crate::pedalkit::eq::Eq;
use crate::pedalkit::helpers::Peak;
use crate::pedalkit::muff::MuffSettings;
use crate::pedalkit::wah::WahSettings;

/// What both stacks share: the same circuit, cranked.
const STACK: Voicing = Voicing {
    hp_in_hz: 35.0, // keeps the low C, drops rumble
    trim_db: 0.0,
    bias: 0.0,
    dc_hz: 30.0,
    inter_lp_hz: 6500.0,
    stage2_db: 8.0,
    bass: Peak::new(100.0, 0.8, 0.0),
    mid: Peak::new(650.0, 0.7, 0.0),
    upper: Peak::new(3000.0, 1.0, 0.0),
    treble_hz: 2000.0,
    treble_min: 0.25,
    treble_span: 1.1,
    power_db: 6.0,
    power_bias: 0.1,
    sag: 0.0,
    sag_attack: 0.015,
    sag_release: 0.2,
    reso: Peak::new(120.0, 0.8, 5.0),
    out: 0.6,
};

/// Orange-style (left): big lows, forward mids around 650 Hz, a bit of bite.
pub const ORANGE: Voicing = Voicing {
    trim_db: 0.0,
    bias: 0.25,
    bass: Peak::new(100.0, 0.8, 4.0),
    mid: Peak::new(650.0, 0.7, 1.5),
    upper: Peak::new(3000.0, 1.0, 1.5),
    inter_lp_hz: 6500.0,
    sag: 0.6,
    ..STACK
};

/// Matamp-style (right): fatter low-mids, a little softer up top and more sag.
pub const MATAMP: Voicing = Voicing {
    trim_db: -2.0,
    bias: 0.15,
    bass: Peak::new(80.0, 0.8, 3.0),
    mid: Peak::new(250.0, 0.9, 3.0),
    upper: Peak::new(1200.0, 1.0, -1.5),
    inter_lp_hz: 5500.0,
    sag: 0.8,
    ..STACK
};

/// Stacked in front of the amps, driving them a few dB harder than the guitar alone.
pub const MUFF: MuffSettings = MuffSettings {
    hp_hz: 60.0,
    gain_db: 28.0,
    lp1_hz: 5000.0,
    stage2: 4.0,
    lp2_hz: 4000.0,
    tone_lo_hz: 400.0,
    tone_lo_mix: 0.6,
    tone_hi_hz: 1200.0,
    tone_hi_mix: 0.5,
    out: 0.35,
    fade: 0.015,
};

/// The lead's wah, parked half-cocked.
pub const WAH: WahSettings =
    WahSettings { park_hz: 1000.0, heel_hz: 400.0, toe_hz: 2200.0, q: 2.5, boost_db: 9.0, fade: 0.015 };

/// 4x12, voiced low for C tuning: a big low resonance, a mid dip, speaker
/// presence and a steep top-end roll-off.
pub const CAB_4X12: [Eq; 6] = [
    Eq::Hp(55.0, 0.707),
    Eq::Peak(100.0, 0.9, 6.0),
    Eq::Peak(500.0, 0.9, -2.5),
    Eq::Peak(1800.0, 1.4, 2.0),
    Eq::Peak(4000.0, 2.0, -4.0),
    Eq::Lp(5000.0, 0.75),
];
