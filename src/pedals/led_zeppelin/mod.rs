//! Led Zeppelin's Jimmy Page tone: the pedal (`Patch`), its blocks and its
//! settings (`presets`). Self-contained apart from `crate::pedalkit`, so
//! superPedal carries an identical copy.

mod echoplex_pre;
mod patch;
mod plexi;
pub mod presets;
mod room;
mod tape_echo;
mod tone_bender;

pub use patch::Patch;
