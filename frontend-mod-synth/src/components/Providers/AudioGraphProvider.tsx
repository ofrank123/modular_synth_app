import React, { createContext } from "react";
import { useCallback } from "react";
import { useState } from "react";
import { Connection } from "../../util/Connection";
import { ModuleData } from "../../util/ModuleData";

export const ModulesContext = createContext<ModuleData[]>([]);
export const ConnectionsContext = createContext<Connection[]>([]);
export const GraphDispatchContext = createContext<
  (
    msg:
      | { type: "addModule"; data: ModuleData }
      | { type: "addConnection"; data: Connection }
      | {
          type: "updateModule";
          data: { id: string; modData: { x_pos?: number; y_pos?: number } };
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

  const dispatch: React.ContextType<typeof GraphDispatchContext> = useCallback(
    (msg) => {
      if (msg.type === "addModule") {
        setModules((oldModules) => [...oldModules, msg.data]);
      } else if (msg.type === "addConnection") {
        setConnections((oldConns) => [...oldConns, msg.data]);
      } else if (msg.type === "updateModule") {
        setModules((oldModules) =>
          oldModules.map((mod) =>
            mod.id === msg.data.id ? { ...mod, ...msg.data.modData } : mod
          )
        );
      }
    },
    [setModules, setConnections]
  );

  return (
    <ModulesContext.Provider value={modules}>
      <ConnectionsContext.Provider value={connections}>
        <GraphDispatchContext.Provider value={dispatch}>
          {children}
        </GraphDispatchContext.Provider>
      </ConnectionsContext.Provider>
    </ModulesContext.Provider>
  );
};
