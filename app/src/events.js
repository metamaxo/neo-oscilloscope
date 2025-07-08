import { updateSetting } from "./settings.js";
import { wasmInterface } from "./wasm.js";
import { changePlaybackSpeed } from "./nodes.js";
import { maybeUpdateAutomation } from "./automationCanvas/automation.js";
import { drawAll } from "./automationCanvas/canvas.js";

// All possible audio effects.
const audioEffectSettings = [
  {
    id: "freqRange",
    key: "freq",
    setter: (val) => changePlaybackSpeed(val),
  },
];

/// Style settings
const styleSettings = [
  { id: "scaleRange", key: "scale" },
  { id: "lineRange", key: "stroke" },
  { id: "persistenceRange", key: "persistence" },
  { id: "hueSlider", key: "hue" },
  { id: "imageOpacity", key: "imageOpacity" },
];

/// Processing settings
const processingSettings = [
  {
    id: "threshold",
    key: "threshold",
    setter: (v) => wasmInterface.set_threshold(v),
  },
  {
    id: "pixThreshold",
    key: "pixThreshold",
    setter: (v) => wasmInterface.set_pix_threshold(v),
  },
  {
    id: "repeat",
    key: "repeat",
    setter: (v) => wasmInterface.set_repeat(v),
    events: ["input", "change"],
  },
  {
    id: "intAmount",
    key: "intAmount",
    setter: (v) => wasmInterface.set_int_amount(v),
    events: ["input", "change"],
  },
];

// Audio effect settings listening event.
audioEffectSettings.forEach(
  ({ id, key, setter, events = ["input", "change"] }) => {
    const el = document.getElementById(id);
    if (!el) return;

    events.forEach((evt) => {
      el.addEventListener(evt, () => {
        const val = parseFloat(el.value);
        if (!isNaN(val)) {
          updateSetting("effects", key, val);
          setter(val);
        }
      });
    });
  },
);

/// Style settings listening event.
styleSettings.forEach(({ id, key, value, events = ["input", "change"] }) => {
  const el = document.getElementById(id);
  if (!el) return;

  events.forEach((evt) => {
    el.addEventListener(evt, () => {
      const val = parseFloat(el.value);
      if (!isNaN(val)) {
        console.log(`updating ${key} to ${val}`);
        updateSetting("style", key, val);
        maybeUpdateAutomation(key);
        drawAll();
      }
    });
  });
});

/// Processing settings listening event.
processingSettings.forEach(({ id, key, setter, events = ["input"] }) => {
  const el = document.getElementById(id);
  events.forEach((evt) => {
    el.addEventListener(evt, async () => {
      const value = parseFloat(el.value, 10);
      if (!Number.isNaN(value)) {
        updateSetting("processing", key, value);
        await setter(value);
        await wasmInterface.process_image_to_coords();
        await wasmInterface.process_coords_to_audio();
      }
    });
  });
});

let mode = document.getElementById("lines");
mode.addEventListener("change", () => {
  if (mode.checked) {
    updateSetting("style", "drawMode", "lines");
  } else {
    updateSetting("style", "drawMode", "dots");
  }
});

const method = document.getElementById("methods");
method.addEventListener("change", async (event) => {
  const selectedValue = event.target.value;
  try {
    await wasmInterface.set_mode(selectedValue);
  } catch (err) {
    console.error("Failed to set mode:", err);
  }
});

document
  .getElementById("edgeDetection")
  .addEventListener("change", async (e) => {
    try {
      await wasmInterface.set_edge_detection(e.target.checked);
      await wasmInterface.process_image_to_coords();
      await wasmInterface.process_coords_to_audio();
    } catch (err) {
      console.error("failed to set edge deteciont");
    }
  });

const clipLength = document.getElementById("clipLength");
clipLength.addEventListener("change", async (event) => {
  const selectedLength = parseInt(clipLength.value);
  updateSetting("automation", "clipLength", selectedLength);
});
