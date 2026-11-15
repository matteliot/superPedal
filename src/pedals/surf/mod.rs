//! Surf: the pedal (`Patch`), its blocks and its settings (`presets`).
//! Self-contained apart from `crate::pedalkit`, so superPedal carries an
//! identical copy. Ported from the original C++ patch (FxPatchSDK), block for
//! block.

mod arm_dive;
mod channel;
mod patch;
mod presets;
mod spank;
mod spring;
mod spring_tank;
mod tank_kick;
mod tape_echo;

pub use patch::Patch;
