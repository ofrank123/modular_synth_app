import React from "react";
import { useAudioContext } from "../../hooks/audioContext";

const TestOsc = (): JSX.Element => {
  const { toggle, state } = useAudioContext();
  return (
    <button onClick={toggle}>{state === "play" ? "Pause" : "Play"}</button>
  );
};

export default TestOsc;
