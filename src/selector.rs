//! The left knob as a selector: its travel split into `n` equal zones, one
//! per position (with 12 positions on a 360-degree knob, a new one every 30
//! degrees). A little hysteresis at each boundary, so a knob resting on one
//! can't flicker between two pedals (and their announcements).

/// How far past a boundary (fraction of the knob's travel) the knob must go
/// before the zone changes.
pub const HYSTERESIS: f32 = 0.01;

pub struct Selector {
    n: usize,
    zone: usize,
}

impl Selector {
    pub const fn new(n: usize) -> Self {
        Self { n, zone: 0 }
    }

    pub fn zone(&self) -> usize {
        self.zone
    }

    /// The zone `v` (0..1) falls in, with no hysteresis.
    pub fn zone_of(&self, v: f32) -> usize {
        ((v.clamp(0.0, 1.0) * self.n as f32) as usize).min(self.n - 1)
    }

    /// Jumps straight to `v`'s zone.
    pub fn set(&mut self, v: f32) {
        self.zone = self.zone_of(v);
    }

    /// Follows the knob; returns the new zone if it changed.
    pub fn update(&mut self, v: f32) -> Option<usize> {
        let z = self.zone_of(v);
        if z == self.zone {
            return None;
        }
        let lo = self.zone as f32 / self.n as f32;
        let hi = (self.zone + 1) as f32 / self.n as f32;
        if v < lo - HYSTERESIS || v >= hi + HYSTERESIS || v <= 0.0 || v >= 1.0 {
            self.zone = z;
            Some(z)
        } else {
            None
        }
    }
}
