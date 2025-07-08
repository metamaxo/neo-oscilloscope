// automation.js
import { settings } from "../settings/settings.js";
import { settingSelect, endX, canvas } from "./constants.js";

let automationData = defaultAutomation();
let currentValue = "hue"; // default

export function getCurrentValue() {
  return currentValue;
}

export function getAutomationData() {
  return automationData;
}

export function resetAutomation() {
  automationData = defaultAutomation(window.settings);
}

export function setCurrentValue(value) {
  currentValue = value;
}

function createPoint(x, y, control = null) {
  return { point: [x, y], control };
}

export function defaultAutomation() {
  let automationData = {
    hue: [],
    scale: [],
    stroke: [],
    persistence: [],
  };

  for (const setting in automationData) {
    let y = getCurrentSettingValue(setting);
    const start = createPoint(0, y);
    const end = createPoint(endX, y, [(0 + endX) / 2, y]);
    automationData[setting].push(start, end);
  }

  return automationData;
}

export function getCurrentSettingValue(setting) {
  const height = canvas.height;
  switch (setting) {
    case "persistence":
      return height * (1 - settings.style.persistence / 99);
    case "hue":
      return height * (1 - settings.style.hue / 360);
    case "scale":
      return height * (1 - (settings.style.scale - 50) / 300);
    case "stroke":
      return height * (1 - (settings.style.stroke - 1) / 4);
    default:
      return height / 2;
  }
}

export function maybeUpdateAutomation(setting) {
  settingSelect.value = setting;
  if (!automationData[setting] || automationData[setting].length > 2) return;

  const y = getCurrentSettingValue(setting);
  automationData[setting] = [
    createPoint(0, y),
    createPoint(canvas.width, y, [canvas.width / 2, y]),
  ];
}
