import React, { useEffect, useRef, useState } from "react";
import { useConnections, useModules } from "../../hooks/audioGraph";
import styles from "./Modules.module.scss";

interface ConnectionProps {
  in_node: string;
  in_port: string;
  out_node: string;
  out_port: string;
  offset_x: number;
  offset_y: number;
}

const Connection = ({
  in_node,
  in_port,
  out_node,
  out_port,
  offset_x,
  offset_y,
}: ConnectionProps): JSX.Element => {
  // Force rerender on modules change
  const modules = useModules();

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
      d={`M${out_x} ${out_y}C${
        out_x + Math.max((in_x - out_x) / 2, 200)
      } ${out_y} ${
        in_x - Math.max((in_x - out_x) / 2, 200)
      } ${in_y} ${in_x} ${in_y}`}
    />
  );
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
      {connections.map(({ in_node, in_port, out_node, out_port }) => (
        <Connection
          key={`${in_node}${in_port}${out_node}${out_port}`}
          {...{ in_node, in_port, out_node, out_port, offset_x, offset_y }}
        />
      ))}
    </svg>
  );
};
