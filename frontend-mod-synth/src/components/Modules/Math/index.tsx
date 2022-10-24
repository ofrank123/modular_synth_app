import React from "react";
import type { MathModuleData } from "../../../util/ModuleData";
import { ModuleBody, ModuleRow, ModuleText } from "../Module";
import { Slider } from "../../Slider";
import { useAudioContext } from "../../../hooks/audioContext";
import { Port } from "../Port";

interface MathProps {
  module: MathModuleData;
}

export const Math = ({ module }: MathProps): JSX.Element => {
  const { sendMessage } = useAudioContext();

  return (
    <ModuleBody>
      <ModuleRow>
        <ModuleText>1</ModuleText>
        <Slider
          min={-128}
          max={128}
          defaultValue={0}
          onChange={(value) => {
            sendMessage({
              type: "update-node-param",
              id: module.id,
              name: "attenuverter1",
              value: value / 128,
            });
          }}
        />
        <ModuleText justify="right">1</ModuleText>
        <Port nodeId={module.id} portId={"In 1"} portType={"IN"} />
        <Port nodeId={module.id} portId={"Out 1"} portType={"OUT"} />
      </ModuleRow>
      <ModuleRow>
        <ModuleText>2</ModuleText>
        <Slider
          min={-128}
          max={128}
          defaultValue={0}
          onChange={(value) => {
            sendMessage({
              type: "update-node-param",
              id: module.id,
              name: "attenuverter2",
              value: value / 128,
            });
          }}
        />
        <ModuleText justify="right">2</ModuleText>
        <Port nodeId={module.id} portId={"In 2"} portType={"IN"} />
        <Port nodeId={module.id} portId={"Out 2"} portType={"OUT"} />
      </ModuleRow>
      <ModuleRow>
        <ModuleText>3</ModuleText>
        <Slider
          min={-128}
          max={128}
          defaultValue={0}
          onChange={(value) => {
            sendMessage({
              type: "update-node-param",
              id: module.id,
              name: "attenuverter3",
              value: value / 128,
            });
          }}
        />
        <ModuleText justify="right">3</ModuleText>
        <Port nodeId={module.id} portId={"In 3"} portType={"IN"} />
        <Port nodeId={module.id} portId={"Out 3"} portType={"OUT"} />
      </ModuleRow>
      <ModuleRow>
        <ModuleText>4</ModuleText>
        <Slider
          min={-128}
          max={128}
          defaultValue={0}
          onChange={(value) => {
            sendMessage({
              type: "update-node-param",
              id: module.id,
              name: "attenuverter4",
              value: value / 128,
            });
          }}
        />
        <ModuleText justify="right">4</ModuleText>
        <Port nodeId={module.id} portId={"In 4"} portType={"IN"} />
        <Port nodeId={module.id} portId={"Out 4"} portType={"OUT"} />
      </ModuleRow>
      <ModuleRow>
        <ModuleText justify="right">Sum</ModuleText>
        <Port nodeId={module.id} portId={"Sum"} portType={"OUT"} />
      </ModuleRow>
    </ModuleBody>
  );
};
