import React, { useEffect } from "react";
import styles from "./Modules.module.scss";
import { Module } from "./Module";
import { useModules } from "../../hooks/audioGraph";
import { ConnectionDrawer } from "./ConnectionDrawer";
import { Controls } from "../Controls";

export const ModuleArea = (): JSX.Element => {
  const modules = useModules();

  return (
    <div style={{ display: "flex" }}>
      <Controls />
      <div className={styles.modArea}>
        {modules.map((modData) => (
          <Module key={modData.id} moduleData={modData} />
        ))}
        <ConnectionDrawer />
      </div>
    </div>
  );
};
