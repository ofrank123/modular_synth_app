import React from "react";
import type { OscillatorModuleData } from "../../../util/ModuleData";
import { Port } from "../Port";
import { ModuleBody, ModuleRow, ModuleText } from "../Module";
import { Slider } from "../../Slider";
import { useAudioContext } from "../../../hooks/audioContext";

interface OscillatorProps {
  module: OscillatorModuleData;
}

const sliderValToPitch = (value: number) => {
  // MIDI note to pitch calculation
  // From Wikipedia
  return Math.pow(2, (value - 69) / 12) * 440;
};

export const Oscillator = ({ module }: OscillatorProps): JSX.Element => {
  const { sendMessage } = useAudioContext();

  return (
    <ModuleBody>
      <ModuleRow>
        <ModuleText>Pitch</ModuleText>
        <Slider
          min={1}
          max={128}
          onChange={(value) => {
            console.log(sliderValToPitch(value));
            sendMessage({
              type: "update-node-param",
              id: module.id,
              name: "frequency",
              value: sliderValToPitch(value),
            });
          }}
        />
      </ModuleRow>
      <ModuleRow>
        <ModuleText justify="right">Audio</ModuleText>
        <Port nodeId={module.id} portId={"Audio"} portType={"OUT"} />
      </ModuleRow>
    </ModuleBody>
  );
};
