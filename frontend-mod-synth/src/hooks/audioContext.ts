import { useContext, useEffect, useRef, useState } from "react";
import { AudioProviderContext } from "../components/Providers/AudioProvider";
import { audioWorkletNodeTransform } from "../util/CustomWorkletNode";

export const useAudioContext = () => useContext(AudioProviderContext);

/**
 * Sets up and wraps the AudioContext API.  Exposes connection state and toggle function.
 */
export const useAudioContextSetup = (): [boolean, () => void] => {
  const [connected, setConnected] = useState(false);
  const [userGestured, setUserGestured] = useState(false);

  const ctx = useRef<AudioContext | null>();
  const testNode = useRef<AudioWorkletNode | null>();

  // Load AudioContext and modules
  useEffect(() => {}, []);

  useEffect(() => {
    if (userGestured) {
      if (testNode.current && ctx.current) {
        if (connected) {
          testNode.current.connect(ctx.current.destination);
        } else {
          testNode.current.disconnect(ctx.current.destination);
        }
      } else {
        // Initial connection
        (async () => {
          // Create AudioContext
          const newCtx = new AudioContext();

          const wasmRes = await fetch("wasm/wasm_mod_synth_bg.wasm");
          const wasmBytes = await wasmRes.arrayBuffer();

          // Load Module
          try {
            await newCtx.audioWorklet.addModule("BundledProcessor.js");
          } catch (err) {
            throw new Error(`Failed to load TestAudioProcessor.js: ${err}`);
          }

          // Add Worklet to audio Graph
          const newTestNode = audioWorkletNodeTransform(
            new AudioWorkletNode(newCtx, "test-processor")
          );

          if (newTestNode.parameters.has("sampleRate")) {
            (newTestNode.parameters.get("sampleRate") as AudioParam).value =
              newCtx.sampleRate;
          }

          newTestNode.init(wasmBytes, newCtx.sampleRate);

          // Connect up nodes
          if (connected) {
            newTestNode.connect(newCtx.destination);
          } else {
            newTestNode.disconnect(newCtx.destination);
          }

          ctx.current = newCtx;
          testNode.current = newTestNode;
        })();
      }
    }
  }, [userGestured, connected]);

  const toggle = () => {
    setConnected((oldConnected) => !oldConnected);
    setUserGestured(true);
  };

  return [connected, toggle];
};
