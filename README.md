# superPedal

A [Polyend Endless](https://polyend.com/endless/) patch with every band pedal in one. Turn the left knob and a voice tells you which pedal you're on.

| Position | Pedal |
|---|---|
| 0 (fully CCW, the default) | No pedal: the dry guitar |
| 1 | [Gojira](https://github.com/matteliot/gojira) |
| 2 | [Harmonics](https://github.com/matteliot/harmonics) |
| 3 | [Iron Butterfly](https://github.com/matteliot/ironButterfly) |
| 4 | [Jesus and Mary Chain](https://github.com/matteliot/jesusMaryChain) |
| 5 | [Led Zeppelin](https://github.com/matteliot/ledZeppelin) |
| 6 | [Mirrorball](https://github.com/matteliot/mirrorball) |
| 7 | [Nirvana](https://github.com/matteliot/nirvana) |
| 8 | [Sabbath](https://github.com/matteliot/sabbath) |
| 9 | [Sleep](https://github.com/matteliot/sleep) |
| 10 | [Soundgarden](https://github.com/matteliot/soundGarden) |
| 11 | [Surf](https://github.com/matteliot/surf) |

The left knob's travel is split into 12 equal zones, one per position. On a 360° knob that's a new pedal every 30°. Each pedal's name is spoken the moment the knob enters its zone, with the music ducked under it.

| Control | Function |
|---|---|
| Left knob | **Pedal**: picks the pedal (see above) |
| Mid knob | The selected pedal's own **left** knob (Gain, Fuzz, Drive, Dist, Drip, Blend...) |
| Right knob | The selected pedal's own **mid** knob (Tone, Room, Space, Reverb, Mids, Echo, Voice...) |
| Footswitch press / hold | The selected pedal's own press and hold |
| LED | The selected pedal's own; dim white for no pedal |

Each pedal's own right knob (mostly Level) stays at its default, and every pedal is trimmed to the same loudness, so switching pedals doesn't jump in volume. Use the amp to set the volume. Each pedal's knobs and footswitch do exactly what they do on its own patch; see its repo for what they are.

A change of pedal fades the old one out in a few milliseconds, starts the new one from rest, clears its delay memory (so no stale echo or reverb tail plays) and fades it in.

## Install

Download [`release/superpedal.endl`](release/superpedal.endl) (on GitHub: open it and click the download button), connect the Endless to your computer over USB and copy the file onto the pedal. Nothing
needs to be built. It takes 357 KB of the patch's 512 KB.

## Build

```sh
rustup target add thumbv7em-none-eabihf --toolchain nightly
rustup component add llvm-tools --toolchain nightly
./build-endl.sh              # build + make target/endless/superpedal.endl, check f64 use, size and the header
./build-endl.sh --deploy     # ...and copy to /Volumes/ENDLESS
```

## Audition on your computer

```sh
cargo sim                    # -> target/render/superpedal_demo.wav
CALIBRATE=1 cargo sim        # ...and print the loudness trims that match every pedal
```

This turns the knob to each position in turn and plays the same phrase through each pedal. A render is in [`samples/superpedal_demo.wav`](samples/superpedal_demo.wav).

## How it's put together

- `src/pedalkit/`: **pedalkit**, the shared building blocks every pedal is made from (effects, DSP, the Endless ABI, entry points, the demo renderer). This is the master copy.
- `src/pedals/<pedal>/`: an exact copy of each pedal's sound code from its own repo, cloned next to this one under the same parent directory.
- `sync.sh`: keeps the copies in step. `./sync.sh kit` pushes pedalkit to every pedal repo; `./sync.sh pedals` pulls every pedal in; `./sync.sh check` confirms they all match.
- `src/rack.rs`: the pedals on the knob, one line each. `src/selector.rs`: the knob zones. `src/super_pedal.rs`: switching, announcing and the knob mapping. `src/presets.rs`: levels and timings.
- `src/prompts/`: the spoken names (made by `make-prompts.sh` with macOS `say`, committed).

Adding a pedal: clone it next to this repo, add it to `sync.sh` and `src/pedals/mod.rs`, run `./sync.sh pedals`, add one line to `rack!` in `src/rack.rs` and to `make-prompts.sh`, run `./make-prompts.sh`, then run `CALIBRATE=1 cargo sim` and put the suggested trim in `src/presets.rs`.

[`ai.md`](ai.md) holds the full rebuild contract.
