import React from "react";
import { AudioProvider } from "./AudioProvider";

export const Providers = ({ children }: { children: React.ReactNode }) => {
  return <AudioProvider>{children}</AudioProvider>;
};
