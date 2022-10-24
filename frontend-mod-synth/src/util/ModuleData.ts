export type ModuleTypes = "oscillator" | "output" | "math";

export type ModuleData =
  | OscillatorModuleData
  | OutputModuleData
  | MathModuleData;

export type ModuleDataBase = {
  id: string;
  x_pos: number;
  y_pos: number;
};

export type OscillatorModuleData = {
  type: "oscillator";
} & ModuleDataBase;

export type MathModuleData = {
  type: "math";
} & ModuleDataBase;

export type OutputModuleData = {
  type: "output";
} & ModuleDataBase;
