import React, { createContext, useCallback, useState } from "react";
import { useAudioContext } from "../../hooks/audioContext";
import { useConnections, useModules } from "../../hooks/audioGraph";

interface NodeConnectorProps {
  children: React.ReactNode;
}

interface NodeConnectorContextI {
  start: {
    node: string;
    port: string;
    type: "IN" | "OUT";
  } | null;
  connect: (node: string, port: string, type: "IN" | "OUT") => void;
}

export const NodeConnectorContext = createContext<NodeConnectorContextI>({
  start: null,
  connect: () => {
    console.log("No Connector Context!");
  },
});

export const NodeConnectorProvider = ({
  children,
}: NodeConnectorProps): JSX.Element => {
  const { sendMessage } = useAudioContext();
  const [start, setStart] = useState<NodeConnectorContextI["start"]>(null);
  const connections = useConnections();
  const nodes = useModules();

  const windowClickListener = useCallback(() => {
    setStart(null);
    window.removeEventListener("click", windowClickListener);
  }, [setStart]);

  const connect: NodeConnectorContextI["connect"] = useCallback(
    (node, port, type) => {
      if (start === null) {
        setStart({ node, port, type });
        window.addEventListener("click", windowClickListener);
      } else {
        if (start.type !== type) {
          // start.type === OUT
          let [out_node, out_port, in_node, in_port] = [
            start.node,
            start.port,
            node,
            port,
          ];
          if (start.type === "IN") {
            [out_node, out_port, in_node, in_port] = [
              node,
              port,
              start.node,
              start.port,
            ];
          }

          // Only add if there's no existing connection
          if (
            nodes.findIndex((value) => value.id === out_node) !== -1 &&
            nodes.findIndex((value) => value.id === in_node) !== -1 &&
            connections.findIndex(
              (value) =>
                value.out_node === out_node &&
                value.out_port === out_port &&
                value.in_node === in_node &&
                value.in_port === in_port
            ) === -1
          ) {
            window.removeEventListener("click", windowClickListener);
            sendMessage({
              type: "add-connection",
              out_node,
              out_port,
              in_node,
              in_port,
            });
          }
          setStart(null);
        }
      }
    },
    [start, setStart, sendMessage, windowClickListener, connections, nodes]
  );

  return (
    <NodeConnectorContext.Provider value={{ start, connect }}>
      {children}
    </NodeConnectorContext.Provider>
  );
};
