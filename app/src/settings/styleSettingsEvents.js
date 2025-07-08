import { wasmInterface } from "../wasm.js";

const styleSettings = [
  {
    id: "scaleRange",
    key: "scale",
    setter: (v) => wasmInterface.set_scale(v),
  },
  {
    id: "lineRange",
    key: "stroke",
    setter: (v) => wasmInterface.set_stroke(v),
  },
  {
    id: "persistenceRange",
    key: "persistence",
    setter: (v) => wasmInterface.set_persistence(v),
    events: ["input", "change"],
  },
  {
    id: "hueSlider",
    key: "hue",
    setter: (v) => wasmInterface.set_hue(v),
    events: ["input", "change"],
  },
  {
    id: "imageOp",
    key: "imageOpacity",
    setter: (v) => wasmInterface.set_image_opacity(v),
    events: ["input", "change"],
  },
  {
    id: "centerX",
    key: "centerX",
    setter: (v) => wasmInterface.set_center_x(v),
    events: ["input", "change"],
  },
  {
    id: "centerY",
    key: "centerY",
    setter: (v) => wasmInterface.set_center_y(v),
    events: ["input", "change"],
  },
  {
    id: "freqRange",
    key: "freq",
    setter: (v) => wasmInterface.set_playback_rate(v),
    events: ["input", "change"],
  },
];

styleSettings.forEach(({ id, key, setter, events = ["input"] }) => {
  const el = document.getElementById(id);
  if (!el) console.warn("Missing element for ID:", id);
  events.forEach((evt) => {
    el.addEventListener(evt, async () => {
      const value = parseFloat(el.value, 10);
      if (!Number.isNaN(value)) {
        await setter(value);
      }
    });
  });
});

document.getElementById("dots").addEventListener("change", async (e) => {
  try {
    wasmInterface.set_dot_mode(e.target.checked);
  } catch (err) {
    console.error("failed to set edge deteciont");
  }
});
