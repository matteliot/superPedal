//! The Jesus and Mary Chain's *Psychocandy* tone: the pedal (`Patch`), its
//! blocks and its settings (`presets`). Self-contained apart from
//! `crate::pedalkit`, so superPedal carries an identical copy.

mod fuzz;
mod patch;
mod presets;

pub use patch::Patch;
