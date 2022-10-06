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

export const useAddModule = (): AddModule => {
  const dispatch = useContext(GraphDispatchContext);
  const moveModule = useMoveModule();

  return useCallback<AddModule>(
    (id, type) => {
      dispatch({
        type: "addModule",
        data: createModule(id, type),
      });

      if (type === "output") {
        moveModule("0", 500, 250);
      }
      if (type === "oscillator") {
        moveModule("1", 500, 0);
      }
    },
    [dispatch, moveModule]
  );
};

type AddConnection = (
  out_node: string,
  out_port: string,
  in_node: string,
  in_port: string
) => void;

export const useAddConnection = (): AddConnection => {
  const dispatch = useContext(GraphDispatchContext);

  return useCallback<AddConnection>(
    (out_node, out_port, in_node, in_port) => {
      dispatch({
        type: "addConnection",
        data: { out_node, out_port, in_node, in_port },
      });
    },
    [dispatch]
  );
};

type MoveModule = (id: string, x_pos: number, y_pos: number) => void;

export const useMoveModule = (): MoveModule => {
  const dispatch = useContext(GraphDispatchContext);
  const modules = useModules();

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
