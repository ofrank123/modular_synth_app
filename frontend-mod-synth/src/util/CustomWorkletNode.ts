import {
  AudioEngineMessageOut,
  AudioEngineMessageIn,
} from "../hooks/audioContext";

export interface CustomWorkletNode extends AudioWorkletNode {
  onOtherMessage: (message: AudioEngineMessageIn) => void;
  init: (
    wasmBytes: ArrayBuffer,
    sampleRate: number,
    onMessage: (message: AudioEngineMessageIn) => void
  ) => void;
  onmessage: (event: MessageEvent<any>) => void;
  sendMessage: (message: AudioEngineMessageOut) => void;
}

// Darkness, evil, villany
// Please don't read you will not respect me
export const audioWorkletNodeTransform = (
  node: AudioWorkletNode
): CustomWorkletNode => {
  const newNode = node as CustomWorkletNode;

  newNode.init = function (wasmBytes, sampleRate, onOtherMessage) {
    this.port.onmessage = (event) => this.onmessage(event.data);
    this.onOtherMessage = onOtherMessage;

    this.port.postMessage({
      type: "send-wasm-module",
      wasmBytes,
      sampleRate,
    });
  };

  newNode.onmessage = function (event) {
    if (event.type === "wasm-module-loaded") {
      this.port.postMessage({
        type: "begin-audio",
      });
    } else {
      this.onOtherMessage(event as AudioEngineMessageIn);
    }
  };

  newNode.onprocessorerror = function (err) {
    console.error(
      `An error from TestAudioProcessor.process() occured: ${
        (err as ErrorEvent).message
      }`
    );
  };

  newNode.sendMessage = function (message: AudioEngineMessageOut) {
    this.port.postMessage(message);
  };

  return newNode;
};
