import React, { useCallback, useEffect, useRef, useState } from "react";
import { useConnections, useModules } from "../../hooks/audioGraph";
import { Connection } from "../../util/Connection";
import styles from "./Modules.module.scss";
import { useReqRemoveConnection } from "../../hooks/engineMessages";
import { useNodeConnector } from "../../hooks/nodeConnector";

interface ConnectionProps {
  connection: Connection;
  offset_x: number;
  offset_y: number;
}

const bezierCurve = (
  out_x: number,
  out_y: number,
  in_x: number,
  in_y: number
) => {
  const curve_offset =
    in_x - out_x > 0 ? (in_x - out_x) / 2 : Math.abs(in_x - out_x);
  return `M${out_x} ${out_y}C${out_x + curve_offset} ${out_y} ${
    in_x - curve_offset
  } ${in_y} ${in_x} ${in_y}`;
};

const Connection = ({
  connection,
  offset_x,
  offset_y,
}: ConnectionProps): JSX.Element => {
  // Force rerender on modules change
  const modules = useModules();
  const removeConnection = useReqRemoveConnection();

  const { id, in_node, in_port, out_node, out_port } = connection;

  const [{ in_x, in_y, out_x, out_y }, setCoords] = useState({
    in_x: 0,
    in_y: 0,
    out_x: 0,
    out_y: 0,
  });

  useEffect(() => {
    const { x: in_x, y: in_y } = document
      .getElementById(`port_${in_node}_${in_port}_IN`)
      ?.getBoundingClientRect() || { x: 0, y: 0 };
    const { x: out_x, y: out_y } = document
      .getElementById(`port_${out_node}_${out_port}_OUT`)
      ?.getBoundingClientRect() || { x: 0, y: 0 };
    setCoords({
      in_x: in_x - offset_x + 3,
      in_y: in_y - offset_y + 9.5,
      out_x: out_x - offset_x + 15,
      out_y: out_y - offset_y + 9.5,
    });
  }, [in_node, in_port, out_node, out_port, offset_x, offset_y, modules]);

  return (
    <path
      onClick={() => {
        removeConnection(id);
      }}
      d={bezierCurve(out_x, out_y, in_x, in_y)}
    />
  );
};

const MouseConnection = ({
  offsetX,
  offsetY,
}: {
  offsetX: number;
  offsetY: number;
}): JSX.Element => {
  const { start } = useNodeConnector();
  const [{ mouse_x, mouse_y }, setMouseCoords] = useState({
    mouse_x: 0,
    mouse_y: 0,
  });

  const [coords, setCoords] = useState<{
    in_x: number;
    in_y: number;
    out_x: number;
    out_y: number;
  } | null>(null);

  const mouseMoveListener = useCallback(
    (ev: MouseEvent) => {
      setMouseCoords({
        mouse_x: ev.clientX,
        mouse_y: ev.clientY,
      });
    },
    [setMouseCoords]
  );

  useEffect(() => {
    window.addEventListener("mousemove", mouseMoveListener);
    return () => {
      window.removeEventListener("mousemove", mouseMoveListener);
    };
  }, [mouseMoveListener]);

  useEffect(() => {
    if (start) {
      if (start.type === "IN") {
        const { x: in_x, y: in_y } = document
          .getElementById(`port_${start.node}_${start.port}_IN`)
          ?.getBoundingClientRect() || { x: 0, y: 0 };
        setCoords({
          in_x: in_x - offsetX + 3,
          in_y: in_y - offsetY + 9.5,
          out_x: mouse_x - offsetX,
          out_y: mouse_y - offsetY,
        });
      } else {
        const { x: out_x, y: out_y } = document
          .getElementById(`port_${start.node}_${start.port}_OUT`)
          ?.getBoundingClientRect() || { x: 0, y: 0 };
        setCoords({
          in_x: mouse_x - offsetX,
          in_y: mouse_y - offsetY,
          out_x: out_x - offsetX + 15,
          out_y: out_y - offsetY + 9.5,
        });
      }
    } else {
      setCoords(null);
    }
  }, [mouse_x, mouse_y, offsetX, offsetY, start]);

  if (start && coords) {
    const { out_x, out_y, in_x, in_y } = coords;
    return (
      <path
        className={styles.mouseConnection}
        d={bezierCurve(out_x, out_y, in_x, in_y)}
      />
    );
  }
  return <></>;
};

export const ConnectionDrawer = (): JSX.Element => {
  const connections = useConnections();

  const ref = useRef<SVGSVGElement | null>(null);
  const [{ offset_x, offset_y }, setOffset] = useState({
    offset_x: 0,
    offset_y: 0,
  });

  useEffect(() => {
    const calcOffset = () => {
      const { x, y } = ref.current?.getBoundingClientRect() || { x: 0, y: 0 };
      setOffset({ offset_x: x, offset_y: y });
    };

    window.addEventListener("resize", calcOffset);
    calcOffset();

    return () => {
      window.removeEventListener("resize", calcOffset);
    };
  }, []);

  return (
    <svg ref={ref} className={styles.connectionDrawer} fill="none">
      {[...connections].map((conn) => (
        <Connection
          key={conn.id}
          connection={conn}
          {...{ offset_x, offset_y }}
        />
      ))}
      <MouseConnection offsetX={offset_x} offsetY={offset_y} />
    </svg>
  );
};
