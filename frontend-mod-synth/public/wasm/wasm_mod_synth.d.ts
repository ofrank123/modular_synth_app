/* tslint:disable */
/* eslint-disable */
/**
*/
export function init_wasm(): void;
/**
*/
export class AudioManager {
  free(): void;
/**
* @param {number} sample_rate
* @returns {AudioManager}
*/
  static new(sample_rate: number): AudioManager;
/**
* @param {number} out_node_idx
* @param {string} out_node_port
* @param {number} in_node_idx
* @param {string} in_node_port
*/
  connect(out_node_idx: number, out_node_port: string, in_node_idx: number, in_node_port: string): void;
/**
* @param {number} edge_idx
*/
  disconnect(edge_idx: number): void;
/**
* @returns {boolean}
*/
  has_message(): boolean;
/**
* @returns {Message}
*/
  next_message(): Message;
/**
* @param {Message} message
*/
  add_message(message: Message): void;
/**
* @returns {number}
*/
  get_output_ptr(): number;
/**
*/
  process(): void;
}
/**
*/
export class Message {
  free(): void;
/**
* @param {string} name
* @returns {Message}
*/
  static new(name: string): Message;
/**
* @param {string} name
* @param {MessageParamData} param
*/
  add_param(name: string, param: MessageParamData): void;
/**
* @returns {string}
*/
  get_name(): string;
/**
* @param {string} field
* @returns {MessageParamData}
*/
  get_data(field: string): MessageParamData;
}
/**
*/
export class MessageParamData {
  free(): void;
/**
* @param {number} f
* @returns {MessageParamData}
*/
  static float(f: number): MessageParamData;
/**
* @param {string} s
* @returns {MessageParamData}
*/
  static string(s: string): MessageParamData;
/**
* @returns {string}
*/
  get_str(): string;
/**
* @returns {number}
*/
  get_flt(): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_audiomanager_free: (a: number) => void;
  readonly audiomanager_new: (a: number) => number;
  readonly audiomanager_connect: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly audiomanager_disconnect: (a: number, b: number) => void;
  readonly audiomanager_has_message: (a: number) => number;
  readonly audiomanager_next_message: (a: number) => number;
  readonly audiomanager_add_message: (a: number, b: number) => void;
  readonly audiomanager_get_output_ptr: (a: number) => number;
  readonly audiomanager_process: (a: number) => void;
  readonly __wbg_messageparamdata_free: (a: number) => void;
  readonly messageparamdata_float: (a: number) => number;
  readonly messageparamdata_string: (a: number, b: number) => number;
  readonly messageparamdata_get_str: (a: number, b: number) => void;
  readonly messageparamdata_get_flt: (a: number) => number;
  readonly __wbg_message_free: (a: number) => void;
  readonly message_new: (a: number, b: number) => number;
  readonly message_add_param: (a: number, b: number, c: number, d: number) => void;
  readonly message_get_name: (a: number, b: number) => void;
  readonly message_get_data: (a: number, b: number, c: number) => number;
  readonly init_wasm: () => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
