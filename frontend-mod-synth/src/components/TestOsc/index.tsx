import React, { useEffect } from "react";
import { useAudioContext } from "../../hooks/audioContext";
import { Oscilloscope } from "../Oscilloscope";

const TestOsc = (): JSX.Element => {
  const { toggle, state, sendMessage } = useAudioContext();

  useEffect(() => {
    console.log("rerender");
  });

  return (
    <>
      <button onClick={toggle}>{state === "play" ? "Pause" : "Play"}</button>
      <Oscilloscope />
    </>
  );
};

export default TestOsc;
