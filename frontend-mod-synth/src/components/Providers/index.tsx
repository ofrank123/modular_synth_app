import React from "react";
import { AudioGraphProvider } from "./AudioGraphProvider";
import { AudioProvider } from "./AudioProvider";

export const Providers = ({ children }: { children: React.ReactNode }) => {
  return (
    <AudioGraphProvider>
      <AudioProvider>{children}</AudioProvider>
    </AudioGraphProvider>
  );
};
