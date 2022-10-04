import React from "react";
import type { OscillatorModuleData } from "../../../util/ModuleData";
import { Port } from "../Port";
import styles from "../Modules.module.scss";
import { ModuleBody, ModuleRow, ModuleText } from "../Module";

interface OscillatorProps {
  module: OscillatorModuleData;
}

export const Oscillator = ({ module }: OscillatorProps): JSX.Element => {
  return (
    <ModuleBody>
      <ModuleRow>
        <ModuleText>Input</ModuleText>
        <ModuleText justify="right">Output</ModuleText>
        <Port nodeId={module.id} portId={"Audio"} portType={"OUT"} />
        <Port nodeId={module.id} portId={"Audio"} portType={"IN"} />
      </ModuleRow>
    </ModuleBody>
  );
};
