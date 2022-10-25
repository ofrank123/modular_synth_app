import React from "react";
import { AudioGraphProvider } from "./AudioGraphProvider";
import { AudioProvider } from "./AudioProvider";
import { NodeConnectorProvider } from "./NodeConnectorProvider";
import { ModuleSpecProvider } from "./ModuleSpecProvider";

export const Providers = ({ children }: { children: React.ReactNode }) => {
  return (
    <ModuleSpecProvider>
      <AudioGraphProvider>
        <AudioProvider>
          <NodeConnectorProvider>{children}</NodeConnectorProvider>
        </AudioProvider>
      </AudioGraphProvider>
    </ModuleSpecProvider>
  );
};
