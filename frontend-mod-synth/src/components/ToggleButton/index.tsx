import React, { useEffect } from "react";
import { useAudioContext } from "../../hooks/audioContext";
import { useReqAddConnection } from "../../hooks/engineMessages";
import { RecordButton } from "../Record";

export const ToggleButton = (): JSX.Element => {
  const { toggle, state } = useAudioContext();

  return (
    <>
      <button
        onClick={() => {
          toggle();
        }}
      >
        {state === "play" ? "Pause" : "Play"}
      </button>
      <RecordButton />
    </>
  );
};
