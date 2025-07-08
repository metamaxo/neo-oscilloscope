import { settings } from "../settings.js";
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

export function defaultAutomation() {
  let automationData = {
    hue: [],
    scale: [],
    stroke: [],
    persistence: [],
  };
  for (const setting in automationData) {
    let currentValue = getCurrentSettingValue(setting);
    console.log("current value: {}", currentValue);
    automationData[setting].push([0, currentValue]);
    automationData[setting].push([endX, currentValue]);
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
  console.log("current value: {y}");
  automationData[setting] = [
    [0, y],
    [canvas.width, y],
  ];
}
