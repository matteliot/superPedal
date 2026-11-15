//! Host-only demo: `cargo sim [OUT.wav]` turns the left knob to every
//! position in turn and plays the same phrase on each (see `score`), driving
//! the patch's entry points like the firmware (see `pedalkit::host`). Prints
//! each position's loudness after the change; `CALIBRATE=1` also prints the
//! `presets::TRIM_DB` that matches every pedal to `presets::TARGET_DB`.

mod score;

use crate::entry::{ENTRY, pedal};
use crate::pedalkit::abi::WORKING_BUFFER_SIZE;
use crate::pedalkit::host::stats::{rms, to_db};
use crate::pedalkit::host::{self, Options, SRU};
use crate::presets::{TARGET_DB, TRIM_DB};
use crate::rack::{NAMES, POSITIONS};

pub fn main() {
    let out = host::out_path("target/render/superpedal_demo.wav");
    let (events, total) = score::score();
    let o = Options { first: "start", input_trim: 0.6, working: WORKING_BUFFER_SIZE as usize };
    let mut log = vec![];
    let r = host::render_with(&ENTRY, events, total, o, &mut || log.push((pedal().active(), pedal().switching())));

    // Every position actually got selected, in order, and settled in time.
    let mut ok = true;
    let block = |s: f64| (s * SRU as f64) as usize / host::BLOCK;
    println!("\n{:<22} {:>9} {:>11} {:>9}", "position", "selected", "level dB", "trim dB");
    let mut suggest = TRIM_DB;
    for p in 0..POSITIONS {
        let start = 0.1 + p as f64 * score::PER_POSITION;
        let settled = start + score::SETTLE;
        let (active, switching) = log[block(settled)];
        let (l, _) = r.section(NAMES[p]).unwrap();
        let a = (score::SETTLE * SRU as f64) as usize;
        let level = to_db(rms(&l[a..]));
        if active != p || switching {
            ok = false;
        }
        if p > 0 {
            suggest[p] = TRIM_DB[p] + (TARGET_DB as f64 - level) as f32;
        }
        println!("{:<22} {:>9} {:>11.1} {:>9.1}", NAMES[p], if active == p && !switching { "yes" } else { "NO" }, level, TRIM_DB[p]);
    }
    if std::env::var_os("CALIBRATE").is_some() {
        let v: Vec<String> = suggest.iter().map(|d| format!("{d:.1}")).collect();
        println!("\nsuggested TRIM_DB: [{}]", v.join(", "));
    }
    host::finish(&ENTRY, r, &out);
    if !ok {
        eprintln!("error: a position wasn't selected in time");
        std::process::exit(1);
    }
}
