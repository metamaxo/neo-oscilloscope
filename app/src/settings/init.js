import { settings } from "./settings.js";
import { wasmInterface } from "../wasm.js";
import { updateState } from "../state.js";
import "./processingSettingsEvents.js";
import "./styleSettingsEvents.js";
import "./startingPoint.js";

let interval = 100; // ms
while (true) {
  const newSettings = await wasmInterface.get_settings_json();
  Object.assign(settings, newSettings);
  updateState(newSettings);
  await new Promise((r) => setTimeout(r, interval));
}
