export type ModuleTypes = "oscillator" | "output";

export type ModuleData = OscillatorModuleData | OutputModuleData;

export type ModuleDataBase = {
  id: string;
  x_pos: number;
  y_pos: number;
};

export type OscillatorModuleData = {
  type: "oscillator";
} & ModuleDataBase;

export type OutputModuleData = {
  type: "output";
} & ModuleDataBase;
