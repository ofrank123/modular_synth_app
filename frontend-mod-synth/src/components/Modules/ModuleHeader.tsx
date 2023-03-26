import React, { useCallback, useEffect, useRef, useState } from "react";
import { useAudioContext } from "../../hooks/audioContext";
import { useMoveModule } from "../../hooks/audioGraph";
import { ModuleData } from "../../util/ModuleData";
import styles from "./Modules.module.scss";

interface ModuleHeaderProps {
  moduleData: ModuleData;
}

export const ModuleHeader = ({
  moduleData,
}: ModuleHeaderProps): JSX.Element => {
  const { id: modId, type: name, x_pos, y_pos } = moduleData;
  const moveModule = useMoveModule();
  const { sendMessage } = useAudioContext();
  const posRef = useRef({ x_pos, y_pos });

  useEffect(() => {
    posRef.current = { x_pos, y_pos };
  }, [x_pos, y_pos]);

  const onMouseDown: React.MouseEventHandler<HTMLDivElement> = useCallback(
    (e) => {
      e.stopPropagation();
      const mouseMoveListener = (e: MouseEvent) => {
        e.stopPropagation();
        moveModule(modId, e.movementX, e.movementY);
      };

      const removeListeners = () => {
        window.removeEventListener("mousemove", mouseMoveListener);
        window.removeEventListener("mouseup", removeListeners);
      };

      window.addEventListener("mousemove", mouseMoveListener);
      window.addEventListener("mouseup", removeListeners);
    },
    [moveModule, modId]
  );

  return (
    <div className={styles.moduleHeader} onMouseDown={onMouseDown}>
      <span>{name}</span>
      {moduleData.type != "output" && (
        <button
          style={{
            background: "none",
            border: "none",
            padding: ".2rem",
            cursor: "pointer",
          }}
          onClick={() => {
            sendMessage({ type: "remove-node", id: moduleData.id });
          }}
        >
          <svg
            style={{ height: 16, width: 16 }}
            focusable="false"
            fill="#fff"
            viewBox="0 0 24 24"
          >
            <path d="M19 6.41 17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"></path>
          </svg>
        </button>
      )}
    </div>
  );
};
