import { setFips } from "crypto";
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

const sliderToEngine = ({
  val,
  steps,
  order,
  range,
  inverts,
}: {
  val: number;
  steps: number;
  order: number;
  range: number;
  inverts: boolean;
}): number => {
  if (inverts) {
    if (val >= steps / 2) {
      return range * Math.pow((2 * val) / steps - 1, order);
    } else {
      return -range * Math.pow(Math.abs((2 * val) / steps - 1), order);
    }
  } else {
    return range * Math.pow(val / steps, order);
  }
};

const engineToSlider = ({
  val,
  steps,
  order,
  range,
  inverts,
}: {
  val: number;
  steps: number;
  order: number;
  range: number;
  inverts: boolean;
}): number => {
  if (inverts) {
    if (val >= 0) {
      return (steps * (Math.pow(val / range, 1 / order) + 1)) / 2;
    } else {
      return (steps * (-Math.pow(Math.abs(val / range), 1 / order) + 1)) / 2;
    }
  } else {
    let res = steps * Math.pow(val / range, 1 / order);
    return res;
  }
};

const RowEl = ({ id, el }: { id: string; el: RowElement }) => {
  const { sendMessage } = useAudioContext();
  switch (el.type) {
    case "TEXT":
      return (
        <ModuleText justify={el.justify.type === "RIGHT" ? "right" : "left"}>
          {el.data}
        </ModuleText>
      );
    case "SLIDER": {
      const { steps, order, range, inverts } = el;

      return (
        <Slider
          min={0}
          defaultValue={engineToSlider({
            val: el.default,
            steps,
            order,
            range,
            inverts,
          })}
          max={el.steps}
          sliderToEngine={(val) =>
            sliderToEngine({ val, steps, order, range, inverts })
          }
          engineToSlider={(val) =>
            engineToSlider({ val, steps, order, range, inverts })
          }
          onChange={(value) => {
            sendMessage({
              type: "update-node-param",
              id,
              name: el.parameter,
              value: sliderToEngine({
                val: value,
                steps,
                order,
                range,
                inverts,
              }),
            });
          }}
        />
      );
    }
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
