//! Soundgarden's tone: the pedal (`Patch`), its blocks and its settings
//! (`presets`). Self-contained apart from `crate::pedalkit`, so superPedal
//! carries an identical copy.

mod fuzz;
mod patch;
mod presets;
mod rotary;

pub use patch::Patch;
