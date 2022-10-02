import React, { useEffect } from "react";
import { useAudioContext } from "../../hooks/audioContext";

export const ToggleButton = (): JSX.Element => {
  const { toggle, state } = useAudioContext();

  return (
    <>
      <button onClick={toggle}>{state === "play" ? "Pause" : "Play"}</button>
    </>
  );
};
