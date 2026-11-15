//! The pedals on the left knob, in order (alphabetical), after "no pedal".
//! To add a pedal: vendor it (`sync.sh`, `pedals/mod.rs`), add one line to
//! `rack!` below, a prompt to `make-prompts.sh`, and its level to
//! `presets::TRIM_DB`.

use crate::pedalkit::abi::Color;
use crate::pedalkit::pedal::Pedal;
use crate::pedalkit::pool::Pool;

/// What superPedal needs of any pedal, object-safe so one `&mut dyn Slot`
/// can stand for whichever is selected.
pub trait Slot {
    /// Back to rest (`Pedal::NEW`) and `init`, taking its memory from `pool`.
    fn restart(&mut self, pool: &mut Pool);
    /// Knob `k`'s default.
    fn default_knob(&self, k: usize) -> f32;
    fn set_knob(&mut self, k: usize, v: f32);
    fn press(&mut self);
    fn hold(&mut self);
    fn tick(&mut self, l: f32, r: f32) -> (f32, f32);
    fn led(&self) -> Color;
    fn block(&mut self);
}

impl<P: Pedal> Slot for P {
    fn restart(&mut self, pool: &mut Pool) {
        *self = P::NEW;
        self.init(pool);
    }
    fn default_knob(&self, k: usize) -> f32 {
        P::PARAMS[k].default
    }
    fn set_knob(&mut self, k: usize, v: f32) {
        Pedal::set_knob(self, k, v);
    }
    fn press(&mut self) {
        Pedal::press(self);
    }
    fn hold(&mut self) {
        Pedal::hold(self);
    }
    #[inline]
    fn tick(&mut self, l: f32, r: f32) -> (f32, f32) {
        Pedal::tick(self, l, r)
    }
    fn led(&self) -> Color {
        Pedal::led(self)
    }
    fn block(&mut self) {
        Pedal::block(self);
    }
}

macro_rules! rack {
    ($($m:ident => $name:literal),* $(,)?) => {
        /// Positions on the left knob: "no pedal" and then each pedal.
        pub const POSITIONS: usize = 1 + [$($name),*].len();

        /// Every position's name ("no pedal" first).
        pub const NAMES: [&str; POSITIONS] = ["No pedal", $($name),*];

        /// Every position's spoken name (16 kHz mu-law, see `make-prompts.sh`).
        pub static PROMPTS: [&[u8]; POSITIONS] =
            [include_bytes!("prompts/none.ulaw"), $(include_bytes!(concat!("prompts/", stringify!($m), ".ulaw"))),*];

        /// The most delay memory any one pedal takes (floats).
        pub const MAX_POOL: usize = {
            let sizes = [$(<crate::pedals::$m::Patch as Pedal>::POOL),*];
            let mut m = 0;
            let mut i = 0;
            while i < sizes.len() {
                if sizes[i] > m {
                    m = sizes[i];
                }
                i += 1;
            }
            m
        };

        /// Every pedal, at rest until it's selected.
        pub struct Rack {
            $($m: crate::pedals::$m::Patch,)*
        }

        impl Rack {
            pub const NEW: Self = Self { $($m: <crate::pedals::$m::Patch as Pedal>::NEW,)* };

            /// The pedal at knob position `pos` (0 is "no pedal": `None`).
            #[inline]
            pub fn get(&mut self, pos: usize) -> Option<&mut dyn Slot> {
                let mut k = 0;
                $(
                    k += 1;
                    if pos == k {
                        return Some(&mut self.$m);
                    }
                )*
                let _ = k;
                None
            }

            /// As `get`, read-only.
            pub fn get_ref(&self, pos: usize) -> Option<&dyn Slot> {
                let mut k = 0;
                $(
                    k += 1;
                    if pos == k {
                        return Some(&self.$m);
                    }
                )*
                let _ = k;
                None
            }
        }
    };
}

rack! {
    gojira => "Gojira",
    harmonics => "Harmonics",
    iron_butterfly => "Iron Butterfly",
    jesus_mary_chain => "Jesus and Mary Chain",
    led_zeppelin => "Led Zeppelin",
    mirrorball => "Mirrorball",
    nirvana => "Nirvana",
    sabbath => "Sabbath",
    sleep => "Sleep",
    sound_garden => "Soundgarden",
    surf => "Surf",
}
