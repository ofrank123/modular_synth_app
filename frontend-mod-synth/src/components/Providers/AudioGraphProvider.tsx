import React, { createContext, useEffect, useRef } from "react";
import { useCallback } from "react";
import { useState } from "react";
import { Connection } from "../../util/Connection";
import { ModuleData } from "../../util/ModuleData";
import { Transform } from "../Modules";

export const ModulesContext = createContext<ModuleData[]>([]);
export const ConnectionsContext = createContext<Connection[]>([]);
export const TranslateContext = createContext<Transform>({
  translate: { x: 0, y: 0 },
  scale: 1,
});
export const GraphDispatchContext = createContext<
  (
    msg:
      | { type: "addModule"; data: ModuleData }
      | { type: "addConnection"; data: Connection }
      | {
          type: "updateModule";
          data: { id: string; modData: { x_pos?: number; y_pos?: number } };
        }
      | {
          type: "removeConnection";
          data: {
            id: string;
          };
        }
      | {
          type: "removeNode";
          data: {
            id: string;
          };
        }
      | {
          type: "translate";
          data: Transform;
        }
  ) => void
>(() => console.log("No context"));

export const AudioGraphProvider = ({
  children,
}: {
  children: React.ReactNode;
}): JSX.Element => {
  const [modules, setModules] = useState<ModuleData[]>([]);
  const [connections, setConnections] = useState<Connection[]>([]);
  const [transform, setTransform] = useState<Transform>({
    translate: { x: 0, y: 0 },
    scale: 1,
  });

  const transformRef = useRef<Transform>(transform);

  useEffect(() => {
    transformRef.current = transform;
  }, [transform]);

  const dispatch: React.ContextType<typeof GraphDispatchContext> = useCallback(
    (msg) => {
      if (msg.type === "addModule") {
        setModules((oldModules) => [
          ...oldModules,
          {
            ...msg.data,
            x_pos:
              (msg.data.x_pos - transformRef.current.translate.x) *
              transformRef.current.scale,
            y_pos:
              (msg.data.y_pos - transformRef.current.translate.y) *
              transformRef.current.scale,
          },
        ]);
      } else if (msg.type === "addConnection") {
        setConnections((oldConns) => [...oldConns, msg.data]);
      } else if (msg.type === "updateModule") {
        setModules((oldModules) =>
          oldModules.map((mod) =>
            mod.id === msg.data.id
              ? {
                  ...mod,
                  ...msg.data.modData,
                  x_pos: mod.x_pos + (msg.data.modData.x_pos ?? 0),
                  y_pos: mod.y_pos + (msg.data.modData.y_pos ?? 0),
                }
              : mod
          )
        );
      } else if (msg.type === "removeConnection") {
        setConnections((oldConns) =>
          oldConns.filter((value) => value.id !== msg.data.id)
        );
      } else if (msg.type === "removeNode") {
        setConnections((oldConns) =>
          oldConns.filter(
            (value) =>
              value.in_node !== msg.data.id && value.out_node !== msg.data.id
          )
        );
        setModules((oldModules) =>
          oldModules.filter((value) => value.id !== msg.data.id)
        );
      } else if (msg.type === "translate") {
        setTransform(msg.data);
      }
    },
    [setModules, setConnections]
  );

  return (
    <ModulesContext.Provider value={modules}>
      <ConnectionsContext.Provider value={connections}>
        <TranslateContext.Provider value={transform}>
          <GraphDispatchContext.Provider value={dispatch}>
            {children}
          </GraphDispatchContext.Provider>
        </TranslateContext.Provider>
      </ConnectionsContext.Provider>
    </ModulesContext.Provider>
  );
};
