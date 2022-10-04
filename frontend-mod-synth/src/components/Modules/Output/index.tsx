import type { OutputModuleData } from "../../../util/ModuleData";
import React from "react";
import { ModuleBody } from "../Module";

interface OutputProps {
  module: OutputModuleData;
}

export const Output = ({ module }: OutputProps): JSX.Element => {
  return <ModuleBody>Output Module</ModuleBody>;
};
