import {
  getAutomationData,
  getCurrentValue,
} from "../automationCanvas/automation.js";
import { pointRadius, canvas, ctx, settingSelect } from "./constants.js";
import { getPlayhead } from "./playhead.js";

const settingColors = {
  hue: "#00ff00",
  scale: "#00aaff",
  stroke: "#ffaa00",
  persistence: "#ff0000",
};

export function getCanvasCoords(e) {
  const rect = canvas.getBoundingClientRect();
  return [e.clientX - rect.left, e.clientY - rect.top];
}

function drawCurve(ctx, p0, c, p1) {
  ctx.moveTo(...p0);
  ctx.quadraticCurveTo(...c, ...p1);
}

export function drawAll() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  Object.entries(getAutomationData()).forEach(([key, segments]) => {
    const baseColor = settingColors[key] || "#999";
    const stroke = key === settingSelect.value ? baseColor : baseColor + "33";

    ctx.strokeStyle = stroke;
    ctx.lineWidth = 2;
    ctx.beginPath();

    for (let i = 0; i < segments.length - 1; i++) {
      const current = segments[i];
      const next = segments[i + 1];
      const c = current.control || midpoint(current.point, next.point);
      drawCurve(ctx, current.point, c, next.point);
    }
    ctx.stroke();

    for (const segment of segments) {
      drawHandle(segment.point, baseColor, true);
      if (segment.control) drawHandle(segment.control, baseColor, false);
    }
  });

  // Draw playhead
  ctx.strokeStyle = "green";
  ctx.lineWidth = 2;
  const playheadX = getPlayhead();
  ctx.beginPath();
  ctx.moveTo(playheadX, 0);
  ctx.lineTo(playheadX, canvas.height);
  ctx.stroke();
}

function drawHandle([x, y], color, isAnchor) {
  ctx.beginPath();
  ctx.arc(x, y, isAnchor ? pointRadius : pointRadius - 2, 0, Math.PI * 2);
  ctx.fillStyle = color;
  ctx.fill();
}

function midpoint([x1, y1], [x2, y2]) {
  return [(x1 + x2) / 2, (y1 + y2) / 2];
}

drawAll();
