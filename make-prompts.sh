#!/bin/sh
# Render the spoken pedal names into src/prompts/<module>.ulaw (16 kHz G.711 mu-law; the patch embeds these
# files) with the shared pedalkit tool. The results are committed, so building doesn't need a Mac; rerun this
# only to change the wording or the voice (VOICE=..., RATE=... for a faster voice and smaller files). Add a
# line here when a pedal is added (see src/rack.rs).
set -eu
cd "$(dirname "$0")"
exec src/pedalkit/tools/make-prompts.sh src/prompts \
    "none=No pedal" \
    "gojira=Gojira" \
    "harmonics=Harmonics" \
    "iron_butterfly=Iron Butterfly" \
    "jesus_mary_chain=Jesus and Mary Chain" \
    "led_zeppelin=Led Zeppelin" \
    "mirrorball=Mirrorball" \
    "nirvana=Nirvana" \
    "sabbath=Sabbath" \
    "sleep=Sleep" \
    "sound_garden=Soundgarden" \
    "surf=Surf"
