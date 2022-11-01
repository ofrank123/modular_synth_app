import { ModuleData } from "../util/ModuleData";

export type AudioEngineMessageOut =
  | {
      type: "update-node-param";
      id: string;
      name: string;
      value: number | string;
    }
  | {
      type: "remove-connection";
      id: string;
    }
  | {
      type: "add-connection";
      in_node: string;
      out_node: string;
      in_port: string;
      out_port: string;
    }
  | {
      type: "add-module";
      modType: string;
    };

export type AudioEngineMessageIn =
  | {
      type: "raw-samples";
      data: number[];
    }
  | {
      type: "mod-specs";
      data: string;
    }
  | {
      type: "node-created";
      node_id: number;
      node_type: ModuleData["type"];
    }
  | {
      type: "node-connected";
      edge_id: number;
      out_node_id: number;
      out_node_port: string;
      in_node_id: number;
      in_node_port: string;
    }
  | {
      type: "connection-removed";
      edge_id: number;
    };
