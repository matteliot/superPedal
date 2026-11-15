//! Gojira's tone: the pedal (`Patch`), its blocks and its settings
//! (`presets`). Self-contained apart from `crate::pedalkit`, so superPedal
//! carries an identical copy.

mod patch;
mod presets;
mod sirius;
mod space;
mod whammy;

pub use patch::Patch;
