//! Sleep's *Dopesmoker* tone: the pedal (`Patch`) and its settings for the
//! pedalkit blocks (`presets`). Self-contained apart from `crate::pedalkit`,
//! so superPedal carries an identical copy.

mod patch;
mod presets;

pub use patch::Patch;
