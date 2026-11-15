//! Mirrorball's tone: the pedal (`Patch`), its blocks and its settings
//! (`presets`). Self-contained apart from `crate::pedalkit`, so superPedal
//! carries an identical copy.

mod bigsby;
mod downtown;
mod patch;
mod peace;
mod presets;
mod sustainer;

pub use patch::Patch;
