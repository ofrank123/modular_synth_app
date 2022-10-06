import type { OutputModuleData } from "../../../util/ModuleData";
import React from "react";
import { ModuleBody, ModuleRow, ModuleText } from "../Module";
import { Port } from "../Port";

interface OutputProps {
  module: OutputModuleData;
}

export const Output = ({ module }: OutputProps): JSX.Element => {
  return (
    <ModuleBody>
      <ModuleRow>
        <ModuleText>Audio</ModuleText>
        <Port nodeId={module.id} portId={"Audio"} portType={"IN"}></Port>
      </ModuleRow>
    </ModuleBody>
  );
};
