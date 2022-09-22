#!/bin/bash
wasm-pack build --target web
rm -rf ../frontend-mod-synth/public/wasm
cp -r pkg/ ../frontend-mod-synth/public
mv ../frontend-mod-synth/public/pkg ../frontend-mod-synth/public/wasm