interface CustomWorkletNode extends AudioWorkletNode {
  init: (wasmBytes: ArrayBuffer, sampleRate: number) => void;
  onmessage: (event: MessageEvent<any>) => void;
}

// Darkness, evil, villany
// Please don't read you will not respect me
export const audioWorkletNodeTransform = (
  node: AudioWorkletNode
): CustomWorkletNode => {
  const newNode = node as CustomWorkletNode;

  newNode.init = function (wasmBytes, sampleRate) {
    this.port.onmessage = (event) => this.onmessage(event.data);

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
    }
  };

  newNode.onprocessorerror = function (err) {
    console.error(`An error from TestAudioProcessor.process() occured: ${err}`);
  };

  return newNode;
};
