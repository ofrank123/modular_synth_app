import { useCallback, useContext, useEffect, useRef, useState } from "react";
import {
  AudioDataContext,
  AudioProviderContext,
} from "../components/Providers/AudioProvider";
import { CustomWorkletNode } from "../util/CustomWorkletNode";
import { ModuleData } from "../util/ModuleData";
import { setupAudio } from "../util/setupAudio";
import { useAddModule, useAddConnection } from "./audioGraph";

export const useAudioContext = () => useContext(AudioProviderContext);
export const useAudioData = () => useContext(AudioDataContext);

export type AudioEngineMessageOut = {
  type: "update-node-param";
  id: string;
  name: string;
  value: number | string;
};

export type AudioEngineMessageIn =
  | {
      type: "raw-samples";
      data: number[];
    }
  | {
      type: "node-created";
      node_id: number;
      node_type: ModuleData["type"];
    }
  | {
      type: "node-connected";
      out_node_id: number;
      out_node_port: string;
      in_node_id: number;
      in_node_port: string;
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

  const addModule = useAddModule();
  const addConn = useAddConnection();

  const onMessage = useCallback(
    (message: AudioEngineMessageIn) => {
      if (message.type == "raw-samples") {
        setAudioData((oldAudioData) => ({
          ...oldAudioData,
          samples: message.data,
        }));
      } else if (message.type == "node-created") {
        // Handle node creation event
        addModule(message.node_id.toString(), message.node_type);
      } else if (message.type == "node-connected") {
        // Handle node connection event
        addConn(
          message.out_node_id.toString(),
          message.out_node_port,
          message.in_node_id.toString(),
          message.in_node_port
        );
      }
    },
    [addModule, addConn]
  );

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
        setupAudio(onMessage, connected).then(
          ({ ctx: newCtx, node: newNode }) => {
            ctx.current = newCtx;
            node.current = newNode;
          }
        );
      }
    }
  }, [userGestured, connected, onMessage]);

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
