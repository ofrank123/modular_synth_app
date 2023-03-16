import React, { useCallback, useEffect, useRef, useState } from "react";
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
      {name}
    </div>
  );
};
