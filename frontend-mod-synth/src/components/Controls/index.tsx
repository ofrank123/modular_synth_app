import React from "react";
import { useReqAddNode } from "../../hooks/engineMessages";
import { useModuleSpecs } from "../../hooks/moduleSpec";
import styles from "./Controls.module.scss";

export const Controls = () => {
  const { moduleSpecs } = useModuleSpecs();
  const addModule = useReqAddNode();

  return (
    <div className={styles.controls}>
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
    </div>
  );
};
