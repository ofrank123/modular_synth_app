import React, { createContext, useState } from "react";
import { AllModules } from "../../util/ModuleSpec/AllModules";

interface ModuleSpecProviderProps {
  children: React.ReactNode;
}

export interface ModuleSpecContextI {
  setModuleSpecs: (newValue: AllModules) => void;
  moduleSpecs: AllModules;
}

export const ModuleSpecContext = createContext<ModuleSpecContextI>({
  setModuleSpecs: () => {
    console.log("No Module Specification Context");
  },
  moduleSpecs: { data: [] },
});

export const ModuleSpecProvider = ({
  children,
}: ModuleSpecProviderProps): JSX.Element => {
  const [moduleSpecs, setModuleSpecs] = useState<AllModules>({ data: [] });

  return (
    <ModuleSpecContext.Provider value={{ moduleSpecs, setModuleSpecs }}>
      {children}
    </ModuleSpecContext.Provider>
  );
};
