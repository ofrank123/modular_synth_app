/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export function __wbg_audiomanager_free(a: number): void;
export function audiomanager_new(a: number): number;
export function audiomanager_connect(a: number, b: number, c: number, d: number, e: number, f: number, g: number): void;
export function audiomanager_disconnect(a: number, b: number): void;
export function audiomanager_has_message(a: number): number;
export function audiomanager_next_message(a: number): number;
export function audiomanager_add_message(a: number, b: number): void;
export function audiomanager_get_output_ptr(a: number): number;
export function audiomanager_process(a: number): void;
export function __wbg_messageparamdata_free(a: number): void;
export function messageparamdata_float(a: number): number;
export function messageparamdata_string(a: number, b: number): number;
export function messageparamdata_get_str(a: number, b: number): void;
export function messageparamdata_get_flt(a: number): number;
export function __wbg_message_free(a: number): void;
export function message_new(a: number, b: number): number;
export function message_add_param(a: number, b: number, c: number, d: number): void;
export function message_get_name(a: number, b: number): void;
export function message_get_data(a: number, b: number, c: number): number;
export function init_wasm(): void;
export function __wbindgen_malloc(a: number): number;
export function __wbindgen_realloc(a: number, b: number, c: number): number;
export function __wbindgen_add_to_stack_pointer(a: number): number;
export function __wbindgen_free(a: number, b: number): void;
