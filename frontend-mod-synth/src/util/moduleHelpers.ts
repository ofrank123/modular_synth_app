import { ModuleData } from "./ModuleData";

const defaultModBase = {
  x_pos: 0,
  y_pos: 0,
};

const defaultModules: Record<ModuleData["type"], Omit<ModuleData, "id">> = {
  oscillator: {
    ...defaultModBase,
    type: "oscillator",
  },
  output: {
    ...defaultModBase,
    type: "output",
  },
};

export const createModule = (
  id: string,
  type: ModuleData["type"]
): ModuleData => {
  return { id, ...defaultModules[type] } as ModuleData;
};
