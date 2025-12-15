/* tslint:disable */
/* eslint-disable */

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly main: (a: number, b: number) => number;
  readonly wasm_bindgen__convert__closures_____invoke__h44007284e3581768: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h05ab954c157f419b: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__hd6fabbba8cda8abb: (a: number, b: number, c: any, d: any) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h52376d027683a3f5: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__hc67086761d7fcac4: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h19614752c0e72023: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h848b310bc2d481ac: (a: number, b: number, c: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h79313935c6f9dbbd: (a: number, b: number) => void;
  readonly wasm_bindgen__closure__destroy__h99c6c37600e27a0d: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
