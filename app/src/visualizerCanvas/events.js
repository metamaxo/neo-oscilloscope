import { canvas } from "./canvas.js";
import { togglePlayback } from "../state.js";
import {
  getPickingStartingPoint,
  setStartPoint,
  startPoint,
  getCanvasCoords,
} from "../settings/startingPoint.js";

let dragActive = false;

canvas.addEventListener("click", (e) => {
  if (getPickingStartingPoint()) {
    const { x, y } = getCanvasCoords(e);
    setStartPoint(x, y); // place the point under cursor
  } else {
    togglePlayback(); // fallback
  }
});

canvas.addEventListener("mousedown", (e) => {
  if (!getPickingStartingPoint()) return;

  const { x, y } = getCanvasCoords(e);
  const dist = Math.hypot(x - startPoint.x, y - startPoint.y);
  if (dist < 10) dragActive = true;
});

canvas.addEventListener("mousemove", (e) => {
  if (!dragActive || !getPickingStartingPoint()) return;

  const { x, y } = getCanvasCoords(e);
  setStartPoint(x, y); // redraw at new location
});

canvas.addEventListener("mouseup", () => {
  dragActive = false;
});
