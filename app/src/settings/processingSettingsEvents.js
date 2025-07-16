import { wasmInterface } from "../wasm.js";
import { updateDirectionPad } from "./directionPad.js";

const processingSettings = [
  {
    id: "threshold",
    key: "threshold",
    setter: (v) => wasmInterface.set_threshold(v),
  },
  {
    id: "edgeThreshold",
    key: "threshold",
    setter: (v) => wasmInterface.set_edge_threshold(v),
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
    id: "snakeStep",
    key: "spread",
    setter: (v) => wasmInterface.set_step_amount(v),
    events: ["input", "change"],
  },
];

processingSettings.forEach(({ id, key, setter, events = ["input"] }) => {
  const el = document.getElementById(id);
  events.forEach((evt) => {
    el.addEventListener(evt, async () => {
      const value = parseFloat(el.value, 10);
      if (!Number.isNaN(value)) {
        setter(value)
          .then(() => reprocess())
          .catch(console.error);
      }
    });
  });
});

document
  .getElementById("spreadType")
  .addEventListener("change", async (event) => {
    try {
      const value = parseInt(event.target.value, 10);
      await wasmInterface.set_spread_type(value);
      await reprocess();
    } catch (err) {
      console.error("failed to set scanType", err);
    }
  });

document
  .getElementById("scanType")
  .addEventListener("change", async (event) => {
    try {
      const value = parseInt(event.target.value, 10);
      await wasmInterface.set_scan_type(value);
      await reprocess();
    } catch (err) {
      console.error("failed to set scanType", err);
    }
  });

document
  .getElementById("scanLineType")
  .addEventListener("change", async (event) => {
    try {
      const value = parseInt(event.target.value, 10);
      await wasmInterface.set_scan_line_type(value);
      await reprocess();
    } catch (err) {
      console.error("failed to set scanType", err);
    }
  });

const method = document.getElementById("methods");
method.addEventListener("change", async (event) => {
  const selectedValue = event.target.value;
  try {
    updateDirectionPad(selectedValue);
    await wasmInterface.reset_directions();
    await wasmInterface.set_method(selectedValue);
    await reprocess();
  } catch (err) {
    console.error("Failed to set mode:", err);
  }
});

document.getElementById("horizontal").addEventListener("change", async (e) => {
  try {
    await wasmInterface.set_horizontal(e.target.checked);
    await reprocess();
  } catch (err) {
    console.error("failed to set edge deteciont");
  }
});

document.getElementById("doubleTrace").addEventListener("change", async (e) => {
  try {
    await wasmInterface.set_double_trace(e.target.checked);
    await reprocess();
  } catch (err) {
    console.error("failed to set edge deteciont");
  }
});

document.getElementById("scramble").addEventListener("change", async (e) => {
  try {
    await wasmInterface.set_scramble(e.target.checked);
    await reprocess();
  } catch (err) {
    console.error("failed to set edge deteciont");
  }
});

document
  .getElementById("edgeDetection")
  .addEventListener("change", async (e) => {
    try {
      await wasmInterface.set_edge_detection(e.target.checked);
      await reprocess();
    } catch (err) {
      console.error("failed to set edge deteciont");
    }
  });

async function reprocess() {
  try {
    await wasmInterface.process_image_to_coords();
    await wasmInterface.process_coords_to_audio();
  } catch (err) {
    console.error("Failed to process image or coords to audio:", err);
  }
}
