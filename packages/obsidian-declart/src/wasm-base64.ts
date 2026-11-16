// esbuild's base64 loader turns the .wasm file into a default-exported string. The generated
// declart_wasm_bg.wasm.d.ts describes the wasm module's exports instead, so tsc types the
// import wrongly and the string type is asserted here.
import base64 from './wasm/declart_wasm_bg.wasm';

export default base64 as unknown as string;
