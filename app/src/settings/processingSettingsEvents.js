import { wasmInterface } from "../wasm.js";

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
  {
    id: "spreadType",
    key: "spread",
    setter: (v) => wasmInterface.set_spread_type(v),
    events: ["input", "change"],
  },
];

processingSettings.forEach(({ id, key, setter, events = ["input"] }) => {
  const el = document.getElementById(id);
  events.forEach((evt) => {
    el.addEventListener(evt, async () => {
      const value = parseFloat(el.value, 10);
      if (!Number.isNaN(value)) {
        await setter(value);
        try {
          await wasmInterface.process_image_to_coords();
          await wasmInterface.process_coords_to_audio();
        } catch (err) {
          console.error("failed to process coords to audio");
        }
      }
    });
  });
});

const method = document.getElementById("methods");
method.addEventListener("change", async (event) => {
  const selectedValue = event.target.value;
  try {
    await wasmInterface.set_method(selectedValue);
    await wasmInterface.process_image_to_coords();
    await wasmInterface.process_coords_to_audio();
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
  await wasmInterface.set_clip_length(parseInt(clipLength.value));
});
