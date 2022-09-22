import React, { createContext, useContext, useEffect, useState } from "react";
import { useAudioContextSetup } from "../../hooks/audioContext";

interface AudioProviderProps {
  children: React.ReactNode;
}

interface AudioProviderContextI {
  toggle: () => void;
  state: "play" | "pause";
}

export const AudioProviderContext = createContext<AudioProviderContextI>({
  toggle: () => {
    console.log("No Audio Context");
  },
  state: "pause",
});

export const AudioProvider = ({
  children,
}: AudioProviderProps): JSX.Element => {
  const [connected, toggle] = useAudioContextSetup();

  return (
    <AudioProviderContext.Provider
      value={{
        toggle,
        state: connected ? "play" : "pause",
      }}
    >
      {children}
    </AudioProviderContext.Provider>
  );
};
