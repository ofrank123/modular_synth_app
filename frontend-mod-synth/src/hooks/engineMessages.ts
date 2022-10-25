import { useCallback } from "react";
import { ModuleTypes } from "../util/ModuleData";
import { useAudioContext } from "./audioContext";

export const useReqAddNode = () => {
  const { sendMessage } = useAudioContext();

  return useCallback(
    (type: ModuleTypes) => {
      sendMessage({ type: "add-module", modType: type });
    },
    [sendMessage]
  );
};

// Request the engine to remove a connection
export const useReqRemoveConnection = () => {
  const { sendMessage } = useAudioContext();

  return useCallback(
    (id: string) => {
      sendMessage({ type: "remove-connection", id });
    },
    [sendMessage]
  );
};

export const useReqAddConnection = () => {
  const { sendMessage } = useAudioContext();

  return useCallback(
    (data: {
      in_node: string;
      in_port: string;
      out_node: string;
      out_port: string;
    }) => {
      sendMessage({ type: "add-connection", ...data });
    },
    [sendMessage]
  );
};
