//! Pitch tracker (YIN): the input decimated 4:1 into a ring, analysed every
//! YIN_HOP decimated samples once the ring holds a full window; the tracked
//! pitch glides to each new estimate.

use super::presets::*;
use crate::pedalkit::dsp::{Biquad, coef};
use crate::pedalkit::pool::{Pool, Ring};

pub struct Pitch {
    dec_lp: [Biquad; 2],
    dec_phase: u32,
    dec_w: usize,
    dec_n: u32,
    hop: u32,
    /// Decimated input ring.
    dec: Ring,
    /// The newest YIN_W + TAU_MAX decimated samples, oldest first.
    win: Ring,
    /// YIN difference function, then its cumulative-mean-normalised form.
    d: Ring,
    f0: f32,
    /// The tracked pitch (Hz), gliding.
    pub f_s: f32,
    c_glide: f32,
}

impl Pitch {
    pub const NEW: Self = Self {
        dec_lp: [Biquad::NEW; 2],
        dec_phase: 0,
        dec_w: 0,
        dec_n: 0,
        hop: 0,
        dec: Ring::NEW,
        win: Ring::NEW,
        d: Ring::NEW,
        f0: 220.0,
        f_s: 220.0,
        c_glide: 1.0,
    };

    pub const POOL: usize = 1024 + 512 + TAU_MAX + 2;

    pub fn init(&mut self, pool: &mut Pool) {
        // Anti-alias for the 4:1 decimation (4th-order Butterworth, 2 kHz).
        self.dec_lp[0].lowpass(2000.0, 0.541);
        self.dec_lp[1].lowpass(2000.0, 1.307);
        self.dec.take(pool, 1024);
        self.win.take(pool, 512);
        self.d.take(pool, TAU_MAX + 2);
        self.c_glide = coef(GLIDE_S);
    }

    /// Feeds one input sample; returns the gliding pitch.
    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let dx = self.dec_lp[0].process(x);
        let dx = self.dec_lp[1].process(dx);
        self.dec_phase += 1;
        if self.dec_phase == DEC {
            self.dec_phase = 0;
            self.dec_w = (self.dec_w + 1) & 1023;
            self.dec.slice()[self.dec_w] = dx;
            self.dec_n = self.dec_n.saturating_add(1);
            self.hop += 1;
            if self.hop >= YIN_HOP && self.dec_n as usize > YIN_W + TAU_MAX {
                self.hop = 0;
                self.yin();
            }
        }
        self.f_s += (self.f0 - self.f_s) * self.c_glide;
        self.f_s
    }

    /// YIN pitch estimate over the newest decimated samples; updates `f0`
    /// when the input is loud enough and clearly pitched.
    fn yin(&mut self) {
        let n = YIN_W + TAU_MAX;
        let start = self.dec_w.wrapping_sub(n - 1);
        let dec = self.dec.slice();
        let win = self.win.slice();
        let mut e = 0.0;
        for i in 0..n {
            let v = dec[(start + i) & 1023];
            win[i] = v;
            e += v * v;
        }
        if e < PITCH_MIN_RMS * PITCH_MIN_RMS * n as f32 {
            return;
        }
        let w = self.win.slice();
        let d = self.d.slice();
        // Difference function, then cumulative-mean normalisation.
        let mut sum = 0.0;
        d[0] = 1.0;
        for tau in 1..=TAU_MAX {
            let mut s = 0.0;
            for j in 0..YIN_W {
                let x = w[j] - w[j + tau];
                s += x * x;
            }
            sum += s;
            d[tau] = if sum > 0.0 { s * tau as f32 / sum } else { 1.0 };
        }
        // First dip under the threshold (then down to its bottom), else the
        // deepest dip if it's still convincing.
        let mut best = 0;
        let mut tau = TAU_MIN;
        while tau <= TAU_MAX {
            if d[tau] < YIN_THRESH {
                while tau < TAU_MAX && d[tau + 1] < d[tau] {
                    tau += 1;
                }
                best = tau;
                break;
            }
            tau += 1;
        }
        if best == 0 {
            let mut m = TAU_MIN;
            for t in TAU_MIN..=TAU_MAX {
                if d[t] < d[m] {
                    m = t;
                }
            }
            if d[m] > YIN_ACCEPT {
                return;
            }
            best = m;
        }
        // Parabolic interpolation around the dip.
        let mut t = best as f32;
        if best > TAU_MIN && best < TAU_MAX {
            let (a, c0, c) = (d[best - 1], d[best], d[best + 1]);
            let den = a - 2.0 * c0 + c;
            if den > 0.0 {
                t += 0.5 * (a - c) / den;
            }
        }
        self.f0 = DSR / t;
    }
}
