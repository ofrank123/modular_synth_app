import React, {
  UIEventHandler,
  useCallback,
  useContext,
  useEffect,
  useRef,
  useState,
} from "react";
import styles from "./Modules.module.scss";
import { Module } from "./Module";
import { useModules, useUpdateGraph } from "../../hooks/audioGraph";
import { ConnectionDrawer } from "./ConnectionDrawer";
import { Controls } from "../Controls";
import { GraphDispatchContext } from "../Providers/AudioGraphProvider";

export interface Transform {
  translate: {
    x: number;
    y: number;
  };
  scale: number;
}

export const ModuleArea = (): JSX.Element => {
  const modules = useModules();
  const { translate } = useUpdateGraph();
  const [areaTransform, setAreaTransform] = useState<Transform>({
    translate: { x: 0, y: 0 },
    scale: 1,
  });

  const [mouseDown, setMouseDown] = useState(false);
  const prevMousePos = useRef({ x: 0, y: 0 });

  useEffect(() => {
    const handleMouseMove = (event: MouseEvent) => {
      if (mouseDown) {
        setAreaTransform(({ translate: { x: oldX, y: oldY }, scale }) => ({
          translate: {
            x: oldX + (event.clientX - prevMousePos.current.x) / scale,
            y: oldY + (event.clientY - prevMousePos.current.y) / scale,
          },
          scale,
        }));
      }

      prevMousePos.current = { x: event.clientX, y: event.clientY };
    };

    window.addEventListener("mousemove", handleMouseMove);

    return () => {
      window.removeEventListener("mousemove", handleMouseMove);
    };
  }, [mouseDown, setAreaTransform]);

  const handleWheel = useCallback(
    (event: WheelEvent) => {
      event.stopPropagation();
      setAreaTransform(({ translate, scale: oldScale }) => ({
        translate,
        scale: oldScale + event.deltaY / 1000,
      }));
    },
    [setAreaTransform]
  );

  useEffect(() => {
    translate(areaTransform);
  }, [translate, areaTransform]);

  return (
    <div style={{ display: "flex" }}>
      <Controls />
      <div
        onMouseDown={() => setMouseDown(true)}
        onMouseUp={() => setMouseDown(false)}
        // @ts-ignore
        onWheel={(e) => handleWheel(e)}
        className={styles.modArea}
      >
        <div
          style={{
            transform: `scale(${areaTransform.scale}) translate(${areaTransform.translate.x}px, ${areaTransform.translate.y}px)`,
          }}
        >
          {modules.map((modData) => (
            <Module key={modData.id} moduleData={modData} />
          ))}
        </div>
        <ConnectionDrawer transform={areaTransform} />
      </div>
    </div>
  );
};
