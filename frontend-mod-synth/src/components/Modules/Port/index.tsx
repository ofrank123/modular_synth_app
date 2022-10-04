import React from "react";
import styles from "../Modules.module.scss";

interface PortProps {
  nodeId: string;
  portId: string;
  portType: "IN" | "OUT";
  style?: React.CSSProperties;
}

export const Port = ({
  nodeId,
  portId,
  portType,
  style,
}: PortProps): JSX.Element => {
  return (
    <div
      className={styles.modulePort}
      id={`port_${nodeId}_${portId}_${portType}`}
      style={{
        right: portType === "OUT" ? "-10px" : undefined,
        left: portType === "IN" ? "-10px" : undefined,
      }}
    >
      <svg
        width="19"
        height="19"
        viewBox="0 0 19 19"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <circle
          cx="9.5"
          cy="9.5"
          r="6.5"
          fill="#8D8D8D"
          stroke="#555555"
          strokeWidth="2"
        />
      </svg>
    </div>
  );
};
