import {
  settingSelect,
  clearCanvasButton,
  pointRadius,
  canvas,
} from "./constants.js";
import { getCanvasCoords } from "./canvas.js";
import {
  setCurrentValue,
  resetAutomation,
  getCurrentSettingValue,
  getAutomationData,
  getCurrentValue,
} from "./automation.js";

let dragging = null; // { type: 'anchor' | 'control', segmentIndex: number, pointIndex: 0 | 1 }

settingSelect.addEventListener("input", () => {
  setCurrentValue(settingSelect.value);
});

clearCanvasButton.addEventListener("click", () => {
  resetAutomation();
});

canvas.addEventListener("contextmenu", (e) => {
  e.preventDefault();
});

canvas.addEventListener("mousedown", (e) => {
  const automationData = getAutomationData();
  const currentValue = getCurrentValue();
  const segments = automationData[currentValue];
  const [x, y] = getCanvasCoords(e);

  if (e.button === 0) {
    // Drag existing point or control
    for (let i = 0; i < segments.length; i++) {
      const { point, control } = segments[i];
      if (distance(point, [x, y]) < pointRadius + 5) {
        dragging = { type: "anchor", segmentIndex: i };
        return;
      }
      if (control && distance(control, [x, y]) < pointRadius + 5) {
        dragging = { type: "control", segmentIndex: i };
        return;
      }
    }

    // Add new point
    const newPoint = [x, y];
    segments.push({ point: newPoint });
    segments.sort((a, b) => a.point[0] - b.point[0]);
  }

  if (e.button === 2) {
    // Right-click to remove point
    e.preventDefault();
    for (let i = 0; i < segments.length; i++) {
      const { point, control } = segments[i];
      if (distance(point, [x, y]) < pointRadius + 5) {
        segments.splice(i, 1);
        return;
      }
      if (control && distance(control, [x, y]) < pointRadius + 5) {
        segments[i].control = null;
        return;
      }
    }
  }
});

canvas.addEventListener("mousemove", (e) => {
  if (!dragging) return;
  const automationData = getAutomationData();
  const currentValue = getCurrentValue();
  const segments = automationData[currentValue];
  const [x, y] = getCanvasCoords(e);

  const segment = segments[dragging.segmentIndex];
  if (dragging.type === "anchor") {
    segment.point = [x, y];
    segments.sort((a, b) => a.point[0] - b.point[0]);
  } else if (dragging.type === "control") {
    segment.control = [x, y];
  }
});

canvas.addEventListener("mouseup", () => {
  dragging = null;
});

canvas.addEventListener("mouseleave", () => {
  dragging = null;
});

function distance([x1, y1], [x2, y2]) {
  return Math.hypot(x1 - x2, y1 - y2);
}
