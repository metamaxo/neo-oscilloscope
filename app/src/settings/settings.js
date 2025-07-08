import { wasmInterface } from "../wasm.js";

export const settings = {
  mode: "full",
  intAmount: 10,
  threshold: 50,
  pixThreshold: 50,
  sample_rate: 44100,
  edge_detecion: true,
  size: 300,
  repeat: 1,

  loop_audio: true,
  playback_rate: 1,
  dot_mode: true,
  scale: 300,
  stroke: 1,
  persistence: 0.05,
  hue: 140,
  lineColor: "",
  imageOpacity: 0,
  noise: 0,
  centerX: 300,
  centerY: 300,
  clipLength: 10,
};

export function updateColor() {
  settings.style.lineColor = `hsl(${settings.style.hue}, 100%, 40%)`;
}

let interval = 50; // ms
while (true) {
  const newSettings = await wasmInterface.get_settings_json();
  Object.assign(settings, newSettings);
  await new Promise((r) => setTimeout(r, interval));
}
