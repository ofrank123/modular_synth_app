import { AudioEngineMessageIn } from "../hooks/audioContext";
import { audioWorkletNodeTransform } from "./CustomWorkletNode";

export const setupAudio = async (
  onMessage: (message: AudioEngineMessageIn) => void,
  connected: boolean
) => {
  // Create AudioContext
  const ctx = new AudioContext();

  const wasmRes = await fetch("wasm/wasm_mod_synth_bg.wasm");
  const wasmBytes = await wasmRes.arrayBuffer();

  // Load Module
  try {
    await ctx.audioWorklet.addModule("AudioProcessor.js");
  } catch (err) {
    throw new Error(`Failed to load AudioProcessor.js: ${err}`);
  }

  // Add Worklet to audio Graph
  const node = audioWorkletNodeTransform(
    new AudioWorkletNode(ctx, "audio-processor")
  );

  if (node.parameters.has("sampleRate")) {
    (node.parameters.get("sampleRate") as AudioParam).value = ctx.sampleRate;
  }

  node.init(wasmBytes, ctx.sampleRate, onMessage);

  // Connect up nodes
  if (connected) {
    node.connect(ctx.destination);
  } else {
    node.disconnect(ctx.destination);
  }

  return { ctx, node };
};
