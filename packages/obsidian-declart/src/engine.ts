import { initSync, render, describe, themes } from './wasm/declart_wasm.js';

export interface Engine {
  render(input: string, theme: string, width?: number, transparent?: boolean): string;
  /** Text equivalent of a declaration; throws on an invalid declaration. */
  describe(input: string): string;
  themes(): string[];
}

/** Initializes the WASM module synchronously from its bytes and returns the engine. */
export function createEngine(wasmBytes: Uint8Array): Engine {
  initSync({ module: wasmBytes });
  return {
    render: (input, theme, width, transparent) =>
      render(input, theme, width ?? undefined, transparent ?? undefined),
    describe: (input) => describe(input),
    themes: () => themes().split(','),
  };
}
