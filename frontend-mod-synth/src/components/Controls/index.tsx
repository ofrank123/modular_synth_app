import React from "react";
import { useReqAddNode } from "../../hooks/engineMessages";
import styles from "./Controls.module.scss";

export const Controls = () => {
  const addModule = useReqAddNode();

  return (
    <div className={styles.controls}>
      <button
        onClick={() => {
          addModule("oscillator");
        }}
      >
        Add Oscillator
      </button>
    </div>
  );
};
