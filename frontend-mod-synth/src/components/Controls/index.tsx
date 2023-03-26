import React from "react";
import { useAudioContext } from "../../hooks/audioContext";
import { useReqAddNode } from "../../hooks/engineMessages";
import { useModuleSpecs } from "../../hooks/moduleSpec";
import { RecordButton } from "../Record";
import styles from "./Controls.module.scss";

export const Controls = () => {
  const { moduleSpecs } = useModuleSpecs();
  const addModule = useReqAddNode();

  const { toggle, state } = useAudioContext();

  return (
    <div className={styles.controls}>
      <h3>Beep Boop</h3>
      <span style={{ paddingTop: "0.5rem" }}>Controls:</span>
      <button
        onClick={() => {
          toggle();
        }}
      >
        {state === "play" ? "Pause" : "Play"}
      </button>
      <RecordButton />
      {moduleSpecs.data.length != 0 && (
        <>
          <span style={{ paddingTop: "0.5rem" }}>Modules:</span>
          {moduleSpecs.data.map((mod) => (
            <button
              key={`addButton_${mod.name}`}
              onClick={() => {
                addModule(mod.name);
              }}
            >
              Add {mod.name[0].toUpperCase() + mod.name.slice(1)} Module
            </button>
          ))}
        </>
      )}
    </div>
  );
};
