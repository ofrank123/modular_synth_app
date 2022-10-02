import React from "react";
import type {
  ModuleData,
  OscillatorModuleData,
  OutputModuleData,
} from "../../util/ModuleData";
import styles from "./Modules.module.scss";
import { Oscillator } from "./Oscillator";
import { Output } from "./Output";

interface ModuleProps {
  moduleData: ModuleData;
}

export const Module = ({ moduleData }: ModuleProps): JSX.Element => {
  const { type, x_pos, y_pos } = moduleData;
  return (
    <div className={styles.module} style={{ top: y_pos, left: x_pos }}>
      <div className={styles.moduleHeader}>{type}</div>
      <div className={styles.moduleBody}>
        {
          {
            oscillator: (
              <Oscillator module={moduleData as OscillatorModuleData} />
            ),
            output: <Output module={moduleData as OutputModuleData} />,
          }[moduleData.type]
        }
      </div>
    </div>
  );
};
