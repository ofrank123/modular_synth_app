import React, { createContext } from "react";
import { AudioData, useAudioContextSetup } from "../../hooks/audioContext";
import { AudioEngineMessageOut } from "../../util/EngineMessages";

interface AudioProviderProps {
  children: React.ReactNode;
}

interface AudioProviderContextI {
  toggle: () => void;
  state: "play" | "pause";
  recorder: MediaRecorder | null;
  sendMessage: (message: AudioEngineMessageOut) => void;
}

export const AudioProviderContext = createContext<AudioProviderContextI>({
  toggle: () => {
    console.log("No Audio Context");
  },
  state: "pause",
  recorder: null,
  sendMessage: () => {
    console.log("No Audio Context");
  },
});

export const AudioDataContext = createContext<AudioData>({
  samples: [],
});

export const AudioProvider = ({
  children,
}: AudioProviderProps): JSX.Element => {
  const { connected, toggle, sendMessage, audioData, recorder } =
    useAudioContextSetup();

  // Memoize to stabilize object values
  const audioContextValue = React.useMemo<AudioProviderContextI>(
    () => ({
      toggle,
      state: connected ? "play" : "pause",
      recorder: recorder,
      sendMessage,
    }),
    [toggle, connected, recorder, sendMessage]
  );

  const audioDataValue = React.useMemo<AudioData>(() => audioData, [audioData]);

  return (
    <AudioProviderContext.Provider value={audioContextValue}>
      <AudioDataContext.Provider value={audioDataValue}>
        {children}
      </AudioDataContext.Provider>
    </AudioProviderContext.Provider>
  );
};
