import init, { JsInterface } from "../pkg/oscilloscope.js";

export async function setupWasm() {
  await init();
  console.log("WASM initialized successfully");

  const wasmInterface = new JsInterface();
  console.log("WasmInterface instance created");

  console.log("wasmInterface:", wasmInterface);

  return wasmInterface;
}

export const wasmInterface = await setupWasm();
