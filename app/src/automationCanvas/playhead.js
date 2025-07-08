import { settings } from "../settings/settings.js";
import { canvas } from "./constants.js";

let playheadX = 0;
let interval = 50; // ms
let isPlaying = false;

export function getPlayhead() {
  return playheadX;
}

export function resetPlayhead() {
  isPlaying = false;
  playheadX = 0;
}

export async function startPlayhead() {
  const clipLength = settings.clipLength;
  const { width } = canvas;
  const duration = clipLength * 1000;
  const steps = duration / interval;
  const stepSize = width / steps;

  playheadX = 0;
  isPlaying = true;

  for (let i = 0; i <= steps; i++) {
    if (!isPlaying) break;

    playheadX += stepSize;
    await new Promise((r) => setTimeout(r, interval));
  }

  playheadX = 0;
}
