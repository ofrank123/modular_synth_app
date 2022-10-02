import type { OutputModuleData } from "../../../util/ModuleData";
import React from "react";

interface OutputProps {
  module: OutputModuleData;
}

export const Output = ({ module }: OutputProps): JSX.Element => {
  return <div>Output Module</div>;
};
