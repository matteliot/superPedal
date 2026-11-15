//! Black Sabbath's Tony Iommi tone: the pedal (`Patch`), its blocks and its
//! settings (`presets`). Self-contained apart from `crate::pedalkit`, so
//! superPedal carries an identical copy. Ported from the original C++ patch
//! (FxPatchSDK), block for block.

mod caravan;
mod heavy;
mod hiss_gate;
mod leslie;
mod oversampler;
mod patch;
mod presets;
mod rangemaster;
mod tone_stack;
mod tube_stage;

pub use patch::Patch;
