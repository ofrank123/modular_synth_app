import { AudioEngineMessageIn } from "../util/EngineMessages";
import {
  audioWorkletNodeTransform,
  CustomWorkletNode,
} from "./CustomWorkletNode";

export const setupAudio = async (
  onMessage: (message: AudioEngineMessageIn) => void,
  connected: boolean
): Promise<{
  ctx: AudioContext;
  node: CustomWorkletNode;
  recorder: MediaRecorder;
}> => {
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
  const mediaStreamNode = new MediaStreamAudioDestinationNode(ctx);
  const recorder = new MediaRecorder(mediaStreamNode.stream);

  // Connect up nodes
  if (connected) {
    node.connect(mediaStreamNode);
    node.connect(ctx.destination);
  } else {
    node.disconnect(ctx.destination);
    node.disconnect(mediaStreamNode);
  }

  return { ctx, node, recorder };
};
