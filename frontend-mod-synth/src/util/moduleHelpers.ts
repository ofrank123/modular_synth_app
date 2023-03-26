import { useTransform } from "../hooks/audioGraph";
import { ModuleData } from "./ModuleData";

export const createModule = (
  id: string,
  type: ModuleData["type"]
): ModuleData => {
  return {
    id,
    type,
    x_pos: 100,
    y_pos: 100,
  } as ModuleData;
};
