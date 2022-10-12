import React, { useEffect } from "react";
import { useAudioContext } from "../../hooks/audioContext";
import { useReqAddConnection } from "../../hooks/engineMessages";

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
    </>
  );
};
