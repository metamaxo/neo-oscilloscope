import { togglePlayback } from "../state.js";
import { wasmInterface } from "../wasm.js";
const canvas = document.getElementById("overlayCanvas");
const ctx = canvas.getContext("2d");

canvas.addEventListener("click", (e) => {
  console.log("canvas clicked");
  if (getPickingStartingPoint()) {
    return;
  } else {
    togglePlayback(); // fallback
  }
});

export let pickingStartingPoint = false;
export let startPoint = defaultStartPoint();
let isDragging = false;

function defaultStartPoint() {
  let startPoint = { x: 0, y: 0 };
  startPoint.x = canvas.width / 2;
  startPoint.y = canvas.height / 2;
  return startPoint;
}

export function setPickingStartingPoint(value) {
  pickingStartingPoint = value;
}

export function getPickingStartingPoint() {
  return pickingStartingPoint;
}

const button = document.getElementById("startPointBtn");

function drawStartPoint() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  ctx.font = "40px Goldman"; // or any font you like
  ctx.fillStyle = "red";
  ctx.textAlign = "center";
  ctx.textBaseline = "middle";
  ctx.fillText("⌖", startPoint.x, startPoint.y);
}

function getcanvasCoords(e) {
  const rect = canvas.getBoundingClientRect();
  const scaleX = canvas.width / rect.width;
  const scaleY = canvas.height / rect.height;
  return {
    x: (e.clientX - rect.left) * scaleX,
    y: (e.clientY - rect.top) * scaleY,
  };
}

function isInsidePoint(mouseX, mouseY) {
  const dx = mouseX - startPoint.x;
  const dy = mouseY - startPoint.y;
  return dx * dx + dy * dy <= 8 * 8;
}

// Button toggle
button.addEventListener("click", async () => {
  if (!pickingStartingPoint) {
    pickingStartingPoint = true;
    button.textContent = "✓";
    button.font = "30px Goldman";
    canvas.style.cursor = "grab";

    if (!startPoint) {
      // Set point in canvas center
      startPoint.x = canvas.width / 2;
      startPoint.y = canvas.height / 2;
      drawStartPoint();
    } else {
      drawStartPoint();
    }
  } else {
    pickingStartingPoint = false;
    button.textContent = "⌖";
    button.font = "30px Goldman";
    canvas.style.cursor = "default";
    isDragging = false;

    try {
      await wasmInterface.set_starting_point(startPoint.x, startPoint.y);
    } catch (err) {
      console.error("Failed to set starting point:", err);
    }

    ctx.clearRect(0, 0, canvas.width, canvas.height);
  }
});

// Handle dragging
canvas.addEventListener("mousedown", (e) => {
  if (!pickingStartingPoint) return;

  const { x, y } = getcanvasCoords(e);
  if (isInsidePoint(x, y)) {
    isDragging = true;
    canvas.style.cursor = "grabbing";
  }
});

canvas.addEventListener("mousemove", (e) => {
  if (!pickingStartingPoint || !isDragging) return;

  const { x, y } = getcanvasCoords(e);
  startPoint.x = x;
  startPoint.y = y;
  drawStartPoint();
});

canvas.addEventListener("mouseup", () => {
  if (pickingStartingPoint && isDragging) {
    isDragging = false;
    canvas.style.cursor = "grab";
  }
});
