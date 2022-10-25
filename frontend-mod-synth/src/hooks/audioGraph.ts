import { useCallback } from "react";
import { useContext } from "react";
import {
  ConnectionsContext,
  GraphDispatchContext,
  ModulesContext,
} from "../components/Providers/AudioGraphProvider";
import { Connection } from "../util/Connection";
import { ModuleData } from "../util/ModuleData";
import { createModule } from "../util/moduleHelpers";

export const useModules = (): ModuleData[] => useContext(ModulesContext);
export const useConnections = (): Connection[] =>
  useContext(ConnectionsContext);

type AddModule = (id: string, type: ModuleData["type"]) => void;
type AddConnection = (data: Connection) => void;
type RemoveConnection = (data: { id: string }) => void;

export const useUpdateGraph = (): {
  addModule: AddModule;
  addConnection: AddConnection;
  removeConnection: RemoveConnection;
} => {
  const dispatch = useContext(GraphDispatchContext);

  const addConnection = useCallback<AddConnection>(
    (data) => {
      dispatch({
        type: "addConnection",
        data,
      });
    },
    [dispatch]
  );

  const addModule = useCallback<AddModule>(
    (id, type) => {
      dispatch({
        type: "addModule",
        data: createModule(id, type),
      });
    },
    [dispatch]
  );

  const removeConnection = useCallback<RemoveConnection>(
    (data) => {
      dispatch({
        type: "removeConnection",
        data,
      });
    },
    [dispatch]
  );

  return { addModule, addConnection, removeConnection };
};

type MoveModule = (id: string, x_pos: number, y_pos: number) => void;
export const useMoveModule = (): MoveModule => {
  const dispatch = useContext(GraphDispatchContext);

  return useCallback<MoveModule>(
    (id, x_pos, y_pos) => {
      dispatch({
        type: "updateModule",
        data: { id, modData: { x_pos, y_pos } },
      });
    },
    [dispatch]
  );
};
