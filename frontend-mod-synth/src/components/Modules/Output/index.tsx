import React from "react";
import { ModuleData } from "../../../util/ModuleData";
import { ModuleBody, ModuleRow, ModuleText } from "../Module";
import { Port } from "../Port";

interface OutputProps {
  module: ModuleData;
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
