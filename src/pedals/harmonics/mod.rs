//! Harmonics: the pedal (`Patch`), its blocks and its settings (`presets`,
//! with the spoken prompts in `prompts/`). Self-contained apart from
//! `crate::pedalkit`, so superPedal carries an identical copy.

mod modes;
mod patch;
mod pitch;
mod presets;

pub use patch::Patch;
