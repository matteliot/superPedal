//! superPedal: a Polyend Endless patch with every band pedal in one. The left
//! knob picks the pedal (its travel split into equal zones: no pedal, then
//! each pedal in alphabetical order) and a voice says which one; the other
//! two knobs and the footswitch play the selected pedal. See `super_pedal`.
//!
//! The pedals live in `pedals` (exact copies of each pedal repo's sound code,
//! kept in step by `sync.sh`), listed for the knob in `rack`; everything is
//! built from `pedalkit`, whose master copy lives here. The entry points come
//! from `pedalkit::endless_patch!`; the pedals' delay memory is the
//! firmware's working buffer, shared, since only one pedal plays at a time.
//! Host builds (`cargo sim`) render a demo through the same code; see `host`.

#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

/// Reusable blocks (effects, DSP, the Endless ABI); the master copy.
mod pedalkit;

/// The vendored pedals.
mod pedals;

mod presets;
mod rack;
mod selector;
mod super_pedal;

endless_patch!(kit = crate::pedalkit, pedal = crate::super_pedal::SuperPedal, memory = working);

#[cfg(not(target_os = "none"))]
mod host;

#[cfg(not(target_os = "none"))]
fn main() {
    host::main();
}
