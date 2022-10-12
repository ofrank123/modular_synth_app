import React from "react";
import { AudioGraphProvider } from "./AudioGraphProvider";
import { AudioProvider } from "./AudioProvider";
import { NodeConnectorProvider } from "./NodeConnectorProvider";

export const Providers = ({ children }: { children: React.ReactNode }) => {
  return (
    <AudioGraphProvider>
      <AudioProvider>
        <NodeConnectorProvider>{children}</NodeConnectorProvider>
      </AudioProvider>
    </AudioGraphProvider>
  );
};
