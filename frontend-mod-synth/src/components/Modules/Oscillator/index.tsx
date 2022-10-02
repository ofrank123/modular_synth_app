import type { OscillatorModuleData } from "../../../util/ModuleData";
import React from "react";

interface OscillatorProps {
  module: OscillatorModuleData;
}

export const Oscillator = ({ module }: OscillatorProps): JSX.Element => {
  return <div>Osc Module</div>;
};
