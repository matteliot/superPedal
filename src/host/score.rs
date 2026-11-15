//! The demo score: the knob turned to each position in turn (no pedal, then
//! every pedal), and at each the same original phrase in E: power chords, a
//! muted chug, a single-note lick with a bend and a ringing chord.

use crate::pedalkit::guitar::Note;
use crate::pedalkit::host::{Ev, Score};
use crate::rack::{NAMES, POSITIONS};

const E2: f32 = 82.41;
const G2: f32 = 98.0;
const A2: f32 = 110.0;
const B3: f32 = 246.94;
const D4: f32 = 293.66;
const E4: f32 = 329.63;
const G3: f32 = 196.0;

/// Seconds at each position: the announcement and change, then the phrase.
pub const SETTLE: f64 = 1.8;
pub const PER_POSITION: f64 = 7.0;

fn power(root: f32, vel: f32) -> Vec<Note> {
    [root, root * 1.4983, root * 2.0].iter().map(|f| Note::new(*f).vel(vel)).collect()
}

/// The left knob's value at the middle of position `p`'s zone.
pub fn knob_for(p: usize) -> f32 {
    (p as f32 + 0.5) / POSITIONS as f32
}

pub fn score() -> (Score, f64) {
    let mut ev: Score = vec![];
    let q = 60.0 / 100.0;
    let mut t = 0.1;
    for (p, name) in NAMES.iter().enumerate() {
        let start = t;
        ev.push((t, Ev::Knob(0, knob_for(p)), name));
        t += SETTLE;
        for root in [E2, G2, A2] {
            ev.push((t, Ev::Play(power(root, 0.9), 0.012, false), ""));
            t += q;
        }
        ev.push((t, Ev::Play(vec![Note::new(E2).vel(0.8).pm()], 0.0, false), ""));
        t += 0.5 * q;
        ev.push((t, Ev::Play(vec![Note::new(E2).vel(0.8).pm()], 0.0, false), ""));
        t += 0.5 * q;
        for (f, beats) in [(G3, 0.5), (B3, 0.5), (D4, 0.5)] {
            ev.push((t, Ev::Play(vec![Note::new(f).vel(0.85)], 0.0, false), ""));
            t += beats * q;
        }
        ev.push((t, Ev::Play(vec![Note::new(D4).vel(0.9).bend(0.05, 0.15, 200.0).vib()], 0.0, false), ""));
        t += 1.5 * q;
        ev.push((t, Ev::Play([E2, B3 / 2.0, E4 / 2.0, G3, B3, E4].iter().map(|f| Note::new(*f).vel(0.8)).collect(), 0.012, false), ""));
        t = start + PER_POSITION;
    }
    (ev, t + 0.5)
}
