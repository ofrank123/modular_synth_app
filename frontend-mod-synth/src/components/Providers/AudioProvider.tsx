import React, { createContext, useContext, useEffect, useState } from "react";
import {
  AudioData,
  AudioEngineMessageOut,
  useAudioContextSetup,
} from "../../hooks/audioContext";

interface AudioProviderProps {
  children: React.ReactNode;
}

interface AudioProviderContextI {
  toggle: () => void;
  state: "play" | "pause";
  sendMessage: (message: AudioEngineMessageOut) => void;
}

export const AudioProviderContext = createContext<AudioProviderContextI>({
  toggle: () => {
    console.log("No Audio Context");
  },
  state: "pause",
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
  const { connected, toggle, sendMessage, audioData } = useAudioContextSetup();

  // Memoize to stabilize object values
  const audioContextValue = React.useMemo<AudioProviderContextI>(
    () => ({
      toggle,
      state: connected ? "play" : "pause",
      sendMessage,
    }),
    [toggle, connected, sendMessage]
  );

  const audioDataValue = React.useMemo<AudioData>(() => audioData, [audioData]);

  return (
    <AudioProviderContext.Provider value={audioContextValue}>
      <AudioDataContext.Provider value={audioData}>
        {children}
      </AudioDataContext.Provider>
    </AudioProviderContext.Provider>
  );
};
