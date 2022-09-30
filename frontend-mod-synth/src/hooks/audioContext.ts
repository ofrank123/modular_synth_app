import { useCallback, useContext, useEffect, useRef, useState } from "react";
import {
  AudioDataContext,
  AudioProviderContext,
} from "../components/Providers/AudioProvider";
import {
  audioWorkletNodeTransform,
  CustomWorkletNode,
} from "../util/CustomWorkletNode";

export const useAudioContext = () => useContext(AudioProviderContext);
export const useAudioData = () => useContext(AudioDataContext);

export type AudioEngineMessageOut = {};

export type AudioEngineMessageIn = {
  type: "raw-samples";
  data: number[];
};

// Some audio data our program may want to access
export interface AudioData {
  samples: number[]; // Most recent sample buffer
}

/**
 * Sets up and wraps the AudioContext API.  Exposes connection state and toggle function.
 */
export const useAudioContextSetup = (): {
  connected: boolean;
  toggle: () => void;
  sendMessage: (message: AudioEngineMessageOut) => void;
  audioData: AudioData;
} => {
  const [connected, setConnected] = useState(false);
  const [userGestured, setUserGestured] = useState(false);
  const [audioData, setAudioData] = useState<AudioData>({
    samples: [],
  });

  const ctx = useRef<AudioContext | null>();
  const node = useRef<CustomWorkletNode | null>();

  const onMessage = (message: AudioEngineMessageIn) => {
    if (message.type == "raw-samples") {
      setAudioData((oldAudioData) => ({
        ...oldAudioData,
        samples: message.data,
      }));
    }
  };

  useEffect(() => {
    if (userGestured) {
      if (node.current && ctx.current) {
        if (connected) {
          node.current.connect(ctx.current.destination);
        } else {
          node.current.disconnect(ctx.current.destination);
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
          const newNode = audioWorkletNodeTransform(
            new AudioWorkletNode(newCtx, "audio-processor")
          );

          if (newNode.parameters.has("sampleRate")) {
            (newNode.parameters.get("sampleRate") as AudioParam).value =
              newCtx.sampleRate;
          }

          newNode.init(wasmBytes, newCtx.sampleRate, onMessage);

          // Connect up nodes
          if (connected) {
            newNode.connect(newCtx.destination);
          } else {
            newNode.disconnect(newCtx.destination);
          }

          ctx.current = newCtx;
          node.current = newNode;
        })();
      }
    }
  }, [userGestured, connected]);

  const toggle = useCallback(() => {
    setConnected((oldConnected) => !oldConnected);
    setUserGestured(true);
  }, []);

  const sendMessage = useCallback((message: AudioEngineMessageOut) => {
    if (node.current) {
      node.current.sendMessage(message);
    }
  }, []);

  return { connected, toggle, sendMessage, audioData };
};
