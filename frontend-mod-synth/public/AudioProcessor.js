import "./TextEncoder.js";
import init, {
  AudioManager,
  Message,
  MessageParamData,
  init_wasm,
} from "./wasm/wasm_mod_synth.js";

export class AudioProcessor extends AudioWorkletProcessor {
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

    this.send_buffer = [];
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
    } else {
      const msg = Message.new(event.type);
      for (const [key, value] of Object.entries(event)) {
        if (key != "type") {
          const param =
            typeof value === "number"
              ? MessageParamData.float(value)
              : MessageParamData.string(value);

          msg.add_param(key, param);
        }
      }
      this.manager.add_message(msg);
    }
  }

  process(_inputs, outputs, parameters) {
    if (this.initialized) {
      while (this.manager.has_message()) {
        this.handleMessage(this.manager.next_message());
      }

      this.manager.process();
      outputs[0].forEach((channel) => {
        for (let i = 0; i < channel.length; i++) {
          channel[i] = this.buffer[i];
        }
      });
      this.send_buffer.push(...this.buffer);
    }
    if (this.send_buffer.length >= 2048) {
      this.port.postMessage({ type: "raw-samples", data: this.send_buffer });
      this.send_buffer = [];
    }
    return true;
  }

  handleMessage(message) {
    const handlers = {
      node_created: () => {
        this.port.postMessage({
          type: "node-created",
          node_id: message.get_data("node_id").get_flt(),
          node_type: message.get_data("node_type").get_str(),
        });
      },
      node_connected: () => {
        this.port.postMessage({
          type: "node-connected",
          out_node_id: message.get_data("out_node_id").get_flt(),
          out_node_port: message.get_data("out_node_port").get_str(),
          in_node_id: message.get_data("in_node_id").get_flt(),
          in_node_port: message.get_data("in_node_port").get_str(),
        });
      },
    };

    handlers[message.get_name()]();
  }
}

registerProcessor("audio-processor", AudioProcessor);
