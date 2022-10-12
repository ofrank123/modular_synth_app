import { useContext } from "react";
import { NodeConnectorContext } from "../components/Providers/NodeConnectorProvider";

export const useNodeConnector = () => {
  return useContext(NodeConnectorContext);
};
