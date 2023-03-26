import { useCallback } from "react";
import { useContext } from "react";
import { Transform } from "../components/Modules";
import {
  ConnectionsContext,
  GraphDispatchContext,
  ModulesContext,
  TranslateContext,
} from "../components/Providers/AudioGraphProvider";
import { Connection } from "../util/Connection";
import { ModuleData } from "../util/ModuleData";
import { createModule } from "../util/moduleHelpers";

export const useModules = (): ModuleData[] => useContext(ModulesContext);
export const useConnections = (): Connection[] =>
  useContext(ConnectionsContext);
export const useTransform = (): Transform => useContext(TranslateContext);

type AddModule = (id: string, type: ModuleData["type"]) => void;
type AddConnection = (data: Connection) => void;
type RemoveConnection = (data: { id: string }) => void;
type RemoveNode = (data: { id: string }) => void;
type Translate = (data: Transform) => void;

export const useUpdateGraph = (): {
  addModule: AddModule;
  addConnection: AddConnection;
  removeConnection: RemoveConnection;
  removeNode: RemoveNode;
  translate: Translate;
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

  const removeNode = useCallback<RemoveConnection>(
    (data) => {
      dispatch({
        type: "removeNode",
        data,
      });
    },
    [dispatch]
  );

  const translate = useCallback<Translate>(
    (data) => {
      dispatch({
        type: "translate",
        data,
      });
    },
    [dispatch]
  );

  return { addModule, addConnection, removeConnection, removeNode, translate };
};

type MoveModule = (id: string, x_diff: number, y_diff: number) => void;
export const useMoveModule = (): MoveModule => {
  const dispatch = useContext(GraphDispatchContext);
  const transform = useTransform();

  return useCallback<MoveModule>(
    (id, x_diff, y_diff) => {
      dispatch({
        type: "updateModule",
        data: {
          id,
          modData: {
            x_pos: x_diff / transform.scale,
            y_pos: y_diff / transform.scale,
          },
        },
      });
    },
    [dispatch, transform]
  );
};
