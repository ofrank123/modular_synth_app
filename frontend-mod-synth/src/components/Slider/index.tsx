import React from "react";
import ReactSlider from "react-slider";
import styles from "./Slider.module.scss";

interface SliderProps {
  min: number;
  max: number;
  defaultValue?: number;
  sliderToEngine: (value: number) => number;
  engineToSlider: (value: number) => number;
  onChange: (value: number) => void;
}

export const Slider = ({
  min,
  max,
  defaultValue,
  sliderToEngine,
  engineToSlider,
  onChange,
}: SliderProps): JSX.Element => {
  return (
    <ReactSlider
      className={styles.slider}
      trackClassName={styles.sliderTrack}
      thumbClassName={styles.thumb}
      defaultValue={defaultValue}
      min={min}
      max={max}
      onChange={(val, _) => onChange(val)}
      marks={
        sliderToEngine(min) == 0 || sliderToEngine(max) == 0
          ? [min, max]
          : [min, engineToSlider(0), max]
      }
      renderMark={(props) => {
        let transform = undefined;
        if (props.style?.left == 0) {
          transform = "translate(-50%, 0)";
        } else if (sliderToEngine(props.key as number) == 0) {
          transform = "translate(1px, 0)";
        }
        return (
          <span
            {...props}
            style={{
              ...props.style,
              top: "-10px",
              fontSize: ".7rem",
              transform,
            }}
          >
            {sliderToEngine(props.key as number)}
          </span>
        );
      }}
    />
  );
};
