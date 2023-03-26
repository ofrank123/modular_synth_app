import { useCallback, useContext, useEffect, useRef, useState } from "react";
import {
  AudioDataContext,
  AudioProviderContext,
} from "../components/Providers/AudioProvider";
import { CustomWorkletNode } from "../util/CustomWorkletNode";
import { AllModules } from "../util/ModuleSpec/AllModules";
import { setupAudio } from "../util/setupAudio";
import { useUpdateGraph } from "./audioGraph";
import { useModuleSpecs } from "./moduleSpec";
import {
  AudioEngineMessageIn,
  AudioEngineMessageOut,
} from "../util/EngineMessages";

export const useAudioContext = () => useContext(AudioProviderContext);
export const useAudioData = () => useContext(AudioDataContext);

// Audio data the frontend may want to access
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
  recorder: MediaRecorder | null;
} => {
  const [connected, setConnected] = useState(false);
  const [userGestured, setUserGestured] = useState(false);
  const [audioData, setAudioData] = useState<AudioData>({
    samples: [],
  });

  const ctx = useRef<AudioContext | null>(null);
  const node = useRef<CustomWorkletNode | null>(null);
  const recorder = useRef<MediaRecorder | null>(null);

  const { addConnection, addModule, removeConnection, removeNode } =
    useUpdateGraph();

  const { setModuleSpecs } = useModuleSpecs();

  const onMessage = useCallback(
    (message: AudioEngineMessageIn) => {
      if (message.type == "raw-samples") {
        setAudioData((oldAudioData) => ({
          ...oldAudioData,
          samples: message.data,
        }));
      } else if (message.type == "mod-specs") {
        // Handle module spec initialization event
        setModuleSpecs(JSON.parse(message.data) as AllModules);
      } else if (message.type == "node-created") {
        // Handle node creation event
        addModule(message.node_id.toString(), message.node_type);
      } else if (message.type == "node-connected") {
        // Handle node connection event
        addConnection({
          id: message.edge_id.toString(),
          out_node: message.out_node_id.toString(),
          out_port: message.out_node_port,
          in_node: message.in_node_id.toString(),
          in_port: message.in_node_port,
        });
      } else if (message.type == "connection-removed") {
        // Handle connection removal event
        removeConnection({
          id: message.edge_id.toString(),
        });
      } else if (message.type == "node-removed") {
        removeNode({
          id: message.node_id.toString(),
        });
      }
    },
    [addModule, addConnection, removeConnection, removeNode, setModuleSpecs]
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
          ({ ctx: newCtx, node: newNode, recorder: newRecorder }) => {
            ctx.current = newCtx;
            node.current = newNode;
            recorder.current = newRecorder;
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

  // MIDI setup

  const onMIDIMessage = useCallback(
    (midiMessage: any) => {
      const type = midiMessage.data[0];
      const note = midiMessage.data[1];
      const velocity = midiMessage.data[2];

      switch (type) {
        case 144: {
          if (velocity > 0) {
            console.log(`NOTE ON: ${note}`);
            sendMessage({
              type: "midi-message",
              messageType: "NOTE_ON",
              note: note,
            });
          } else {
            console.log(`NOTE OFF: ${note}`);
            sendMessage({
              type: "midi-message",
              messageType: "NOTE_OFF",
              note: note,
            });
          }
          break;
        }
        case 128: {
          console.log(`NOTE OFF: ${note}`);
          sendMessage({
            type: "midi-message",
            messageType: "NOTE_OFF",
            note: note,
          });
          break;
        }
      }
    },
    [sendMessage]
  );

  useEffect(() => {
    //@ts-ignore
    if (navigator.requestMIDIAccess) {
      // Midi supported
      const onMIDISuccess = (midiAccess: any) => {
        for (var input of midiAccess.inputs.values()) {
          input.onmidimessage = onMIDIMessage;
        }
      };

      //@ts-ignore
      navigator.requestMIDIAccess().then(onMIDISuccess, () => {
        console.log("Could not access your MIDI devices");
      });
    } else {
      console.log("Midi not supported");
    }
  }, [onMIDIMessage]);

  return {
    connected,
    toggle,
    sendMessage,
    audioData,
    recorder: recorder.current,
  };
};
