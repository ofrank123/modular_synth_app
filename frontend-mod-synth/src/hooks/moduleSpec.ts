import { useContext } from "react";
import {
  ModuleSpecContext,
  ModuleSpecContextI,
} from "../components/Providers/ModuleSpecProvider";

export const useModuleSpecs = (): ModuleSpecContextI => {
  return useContext(ModuleSpecContext);
};
