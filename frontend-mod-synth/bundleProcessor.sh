#!/bin/bash
npx rollup public/TestAudioProcessor.js --file public/BundledProcessor.js
sed -i '1s/^/\/* eslint-disable *\/\n/' public/BundledProcessor.js