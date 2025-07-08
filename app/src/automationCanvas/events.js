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

let draggingPoint = null;

// Select automation setting
settingSelect.addEventListener("input", (e) => {
  setCurrentValue(settingSelect.value);
});

// clear entire canvas
clearCanvasButton.addEventListener("click", () => {
  resetAutomation();
});

// prevent default context menu
canvas.addEventListener("contextmenu", (e) => {
  e.preventDefault();
});

// left click adds point, right click removes
canvas.addEventListener("mousedown", (e) => {
  let automationData = getAutomationData();
  let currentValue = getCurrentValue();
  const [x, y] = getCanvasCoords(e);
  const points = automationData[currentValue];

  if (e.button === 0) {
    // Left-click: add or drag
    for (let i = 0; i < points.length; i++) {
      const [px, py] = points[i];
      if (Math.hypot(px - x, py - y) < pointRadius + 10) {
        draggingPoint = { index: i };
        return;
      }
    }

    const hasStart = points.some((point) => point[0] <= 1);
    if (!hasStart) {
      const yStart = getCurrentSettingValue(currentValue, window.settings);
      points.push([0, yStart]);
    }

    points.push([x, y]);
    points.sort((a, b) => a[0] - b[0]);
  }

  if (e.button === 2) {
    // Right-click: remove
    e.preventDefault();
    for (let i = 0; i < points.length; i++) {
      const [px2, py2] = points[i];
      if (Math.hypot(px2 - x, py2 - y) < pointRadius + 10) {
        points.splice(i, 1);
        break;
      }
    }
  }
});

canvas.addEventListener("mousemove", (e) => {
  if (!draggingPoint) return;
  let automationData = getAutomationData();
  let currentValue = getCurrentValue();
  const [x, y] = getCanvasCoords(e);
  automationData[currentValue][draggingPoint.index] = [x, y];
  automationData[currentValue].sort((a, b) => a[0] - b[0]);
});

canvas.addEventListener("mouseup", () => {
  draggingPoint = null;
});

canvas.addEventListener("mouseleave", () => {
  draggingPoint = null;
});
