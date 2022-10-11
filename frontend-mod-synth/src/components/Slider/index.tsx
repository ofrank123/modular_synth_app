import React from "react";
import ReactSlider from "react-slider";
import styles from "./Slider.module.scss";

interface SliderProps {
  min: number;
  max: number;
  onChange: (value: number) => void;
}

export const Slider = ({ min, max, onChange }: SliderProps): JSX.Element => {
  return (
    <ReactSlider
      className={styles.slider}
      trackClassName={styles.sliderTrack}
      thumbClassName={styles.thumb}
      min={min}
      max={max}
      onChange={(val, _) => onChange(val)}
    />
  );
};
