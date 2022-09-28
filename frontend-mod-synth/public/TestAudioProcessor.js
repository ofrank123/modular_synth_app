import "./TextEncoder.js";
import init, { AudioManager, init_wasm } from "./wasm/wasm_mod_synth.js";

export class TestAudioProcessor extends AudioWorkletProcessor {
  initialized = false;

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
      init(WebAssembly.compile(event.wasmBytes)).then((result) => {
        init_wasm();
        this.memory = result.memory;
        this.manager = AudioManager.new(event.sampleRate);
        this.buffer_ptr = this.manager.get_output_ptr();
        this.buffer = new Float32Array(
          this.memory.buffer,
          this.buffer_ptr,
          128
        );
        this.port.postMessage({ type: "wasm-module-loaded" });
      });
    } else if (event.type === "begin-audio") {
      this.initialized = true;
    }
  }

  process(_inputs, outputs, parameters) {
    if (this.initialized) {
      this.manager.process();
      outputs[0].forEach((channel) => {
        for (let i = 0; i < channel.length; i++) {
          channel[i] = this.buffer[i];
        }
      });
    }
    return true;
  }
}

registerProcessor("test-processor", TestAudioProcessor);
