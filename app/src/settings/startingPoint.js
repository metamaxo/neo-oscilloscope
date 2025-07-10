import { wasmInterface } from "../wasm.js";
const canvas = document.getElementById("overlayCanvas");
const ctx = canvas.getContext("2d");

let pointPlaced = false;

// interactionState.js
export let pickingStartingPoint = false;

export function setPickingStartingPoint(value) {
  pickingStartingPoint = value;
}

export function getPickingStartingPoint() {
  return pickingStartingPoint;
}

export let startPoint = { x: 0, y: 0 }; // make this exportable

const button = document.getElementById("startPointBtn");

// Draw the draggable point
function drawStartPoint() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  ctx.beginPath();
  ctx.arc(startPoint.x, startPoint.y, 6, 0, 2 * Math.PI);
  ctx.fillStyle = "red";
  ctx.fill();
}
// startingPoint.js
export function setStartPoint(x, y) {
  startPoint.x = x;
  startPoint.y = y;

  drawStartPoint();
}

export function getCanvasCoords(e) {
  const rect = canvas.getBoundingClientRect();
  const scaleX = canvas.width / rect.width;
  const scaleY = canvas.height / rect.height;
  return {
    x: (e.clientX - rect.left) * scaleX,
    y: (e.clientY - rect.top) * scaleY,
  };
}

// Toggle pick/set button behavior
button.addEventListener("click", async () => {
  if (!pickingStartingPoint) {
    pickingStartingPoint = true;
    button.textContent = "Set Starting Point";
    canvas.style.cursor = "crosshair";
    pointPlaced = false;
  } else {
    pickingStartingPoint = false;
    button.textContent = "Pick Starting Point";
    canvas.style.cursor = "default";

    try {
      await wasmInterface.set_starting_point(startPoint.x, startPoint.y);
    } catch (err) {
      console.error("Failed to set starting point:", err);
    }

    ctx.clearRect(0, 0, canvas.width, canvas.height);
  }
});
