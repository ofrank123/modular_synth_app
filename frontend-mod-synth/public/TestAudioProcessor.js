import "./TextEncoder.js";
import init, { get_sine_wave, AudioManager } from "./wasm/wasm_mod_synth.js";

export class TestAudioProcessor extends AudioWorkletProcessor {
  initialized = false;
  manager = undefined;

  static get parameterDescriptors() {
    return [
      {
        name: "sampleRate",
        defaultValue: 48000,
      },
      {
        name: "frequency",
        defaultValue: 440,
      },
    ];
  }

  constructor() {
    super();

    this.port.onmessage = (event) => this.onmessage(event.data);
  }

  onmessage(event) {
    if (event.type === "send-wasm-module") {
      init(WebAssembly.compile(event.wasmBytes)).then(() => {
        this.manager = AudioManager.new(event.sampleRate);

        this.port.postMessage({ type: "wasm-module-loaded" });
      });
    } else if (event.type === "begin-audio") {
      this.initialized = true;
    }
  }

  process(_inputs, outputs, parameters) {
    if (this.initialized) {
      outputs[0].forEach((channel) => {
        const samples = this.manager.get_samples(channel.length);
        for (let i = 0; i < channel.length; i++) {
          channel[i] = samples[i];
        }
      });
    }
    return true;
  }
}

registerProcessor("test-processor", TestAudioProcessor);
