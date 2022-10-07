import React from "react";
import type {
  ModuleData,
  OscillatorModuleData,
  OutputModuleData,
} from "../../util/ModuleData";
import { ModuleHeader } from "./ModuleHeader";
import styles from "./Modules.module.scss";
import { Oscillator } from "./Oscillator";
import { Output } from "./Output";

interface ModuleTextProps {
  children: React.ReactNode;
  justify?: "left" | "right";
}

export const ModuleText = ({
  justify = "left",
  children,
}: ModuleTextProps): JSX.Element => {
  return (
    <div
      style={{
        marginLeft: justify === "right" ? "auto" : undefined,
        paddingRight: justify === "left" ? ".25rem" : undefined,
      }}
    >
      {children}
    </div>
  );
};

interface ModuleRowProps {
  children: React.ReactNode;
}

export const ModuleRow = ({ children }: ModuleRowProps): JSX.Element => {
  return <div className={styles.moduleBodyRow}>{children}</div>;
};

interface ModuleBodyProps {
  children: React.ReactNode;
}

export const ModuleBody = ({ children }: ModuleBodyProps): JSX.Element => {
  return <div className={styles.moduleBody}>{children}</div>;
};

interface ModuleProps {
  moduleData: ModuleData;
}

export const Module = ({ moduleData }: ModuleProps): JSX.Element => {
  const { type, x_pos, y_pos } = moduleData;
  return (
    <div className={styles.module} style={{ top: y_pos, left: x_pos }}>
      <ModuleHeader moduleData={moduleData} />
      {
        {
          oscillator: (
            <Oscillator module={moduleData as OscillatorModuleData} />
          ),
          output: <Output module={moduleData as OutputModuleData} />,
        }[moduleData.type]
      }
    </div>
  );
};
