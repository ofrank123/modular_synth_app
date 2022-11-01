import React from "react";
import { useAudioContext } from "../../hooks/audioContext";
import { useModuleSpecs } from "../../hooks/moduleSpec";
import type { ModuleData } from "../../util/ModuleData";
import { RowElement } from "../../util/ModuleSpec/RowElement";
import { Slider } from "../Slider";
import { ModuleBody, ModuleRow, ModuleText } from "./Module";
import { Port } from "./Port";

interface GenericModProps {
  modData: ModuleData;
}

const RowEl = ({ id, el }: { id: string; el: RowElement }) => {
  const { sendMessage } = useAudioContext();

  switch (el.type) {
    case "TEXT":
      return (
        <ModuleText justify={el.justify.type === "RIGHT" ? "right" : "left"}>
          {el.data}
        </ModuleText>
      );
    case "SLIDER":
      return (
        <Slider
          min={el.min}
          defaultValue={el.default}
          max={el.max}
          onChange={(value) => {
            sendMessage({
              type: "update-node-param",
              id,
              name: el.parameter,
              value,
            });
          }}
        />
      );
    case "SELECTOR":
      return (
        <select
          name={el.parameter}
          id={el.parameter}
          onChange={(event) => {
            sendMessage({
              type: "update-node-param",
              id,
              name: el.parameter,
              value: event.target.value,
            });
          }}
        >
          {el.options.map((opt) => (
            <option key={`${id}_opt_${opt.value}`} value={opt.value}>
              {opt.name}
            </option>
          ))}
        </select>
      );
  }
};

export const GenericMod = ({
  modData: { id, type },
}: GenericModProps): JSX.Element => {
  const { moduleSpecs } = useModuleSpecs();

  const spec = moduleSpecs.data.find((s) => s.name === type);

  if (!spec) {
    return <></>;
  }

  console.log(moduleSpecs);

  return (
    <ModuleBody>
      {spec.rows.map((row, row_idx) => (
        <ModuleRow key={`${id}_row_${row_idx}`}>
          {row.elements.map((el, i) => (
            <RowEl key={`${id}_row_${row_idx}_rowel_${i}`} {...{ id, el }} />
          ))}
          {row.input ? (
            <Port nodeId={id} portId={row.input} portType={"IN"} />
          ) : null}
          {row.output ? (
            <Port nodeId={id} portId={row.output} portType={"OUT"} />
          ) : null}
        </ModuleRow>
      ))}
    </ModuleBody>
  );
};
