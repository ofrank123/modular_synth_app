import React, { useEffect } from "react";
import styles from "./Modules.module.scss";
import { Module } from "./Module";
import { useModules } from "../../hooks/audioGraph";
import { ConnectionDrawer } from "./ConnectionDrawer";

export const ModuleArea = (): JSX.Element => {
  const modules = useModules();

  return (
    <div className={styles.modArea}>
      {modules.map((modData) => (
        <Module key={modData.id} moduleData={modData} />
      ))}
      <ConnectionDrawer />
    </div>
  );
};
