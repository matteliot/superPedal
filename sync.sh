#!/bin/sh
# Keeps the vendored copies in step. superPedal holds the master copy of pedalkit (src/pedalkit); each pedal
# repo (a sibling directory, ../<repo>) owns its sound code in src/<module>/, which superPedal carries as
# src/pedals/<module>/.
#   ./sync.sh check    (default) report any copy that differs; exit 1 if one does
#   ./sync.sh kit      copy src/pedalkit into every pedal repo
#   ./sync.sh pedals   copy every pedal's src/<module>/ into src/pedals/<module>/
# Add a pedal: one "repo:module" line in PEDALS (alphabetical: that's the order on the knob).
set -eu
cd "$(dirname "$0")"
PEDALS="
gojira:gojira
harmonics:harmonics
ironButterfly:iron_butterfly
jesusMaryChain:jesus_mary_chain
ledZeppelin:led_zeppelin
mirrorball:mirrorball
nirvana:nirvana
sabbath:sabbath
sleep:sleep
soundGarden:sound_garden
surf:surf
"
mode="${1:-check}"
bad=0
for p in $PEDALS; do
    repo="../${p%%:*}"
    module="${p#*:}"
    [ -d "$repo" ] || { echo "missing: $repo (clone it next to superPedal)" >&2; bad=1; continue; }
    case "$mode" in
        kit)
            rm -rf "$repo/src/pedalkit" && cp -R src/pedalkit "$repo/src/pedalkit"
            echo "kit -> $repo/src/pedalkit" ;;
        pedals)
            [ -d "$repo/src/$module" ] || { echo "missing: $repo/src/$module" >&2; bad=1; continue; }
            mkdir -p src/pedals && rm -rf "src/pedals/$module" && cp -R "$repo/src/$module" "src/pedals/$module"
            echo "$repo/src/$module -> src/pedals/$module" ;;
        check)
            if ! diff -rq src/pedalkit "$repo/src/pedalkit" > /dev/null 2>&1; then
                echo "differs: $repo/src/pedalkit"; bad=1
            fi
            if ! diff -rq "src/pedals/$module" "$repo/src/$module" > /dev/null 2>&1; then
                echo "differs: src/pedals/$module vs $repo/src/$module"; bad=1
            fi ;;
        *) echo "usage: $0 [check|kit|pedals]" >&2; exit 2 ;;
    esac
done
[ "$mode" = check ] && [ "$bad" = 0 ] && echo "all copies match"
exit "$bad"
