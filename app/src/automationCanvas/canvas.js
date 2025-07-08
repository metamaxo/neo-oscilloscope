import { getAutomationData } from "./automation.js";
import { pointRadius, canvas, ctx, settingSelect } from "./constants.js";
import { getPlayhead } from "./playhead.js";

const settingColors = {
  hue: "#00ff00", // red
  scale: "#00aaff", // light blue
  stroke: "#ffaa00", // orange
  persistence: "#ff0000", // green
};

export function getCanvasCoords(e) {
  const rect = canvas.getBoundingClientRect();
  return [e.clientX - rect.left, e.clientY - rect.top];
}

export function drawAll() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  Object.entries(getAutomationData()).forEach(([key, points]) => {
    const baseColor = settingColors[key] || "#999"; // fallback color
    ctx.strokeStyle =
      key === settingSelect.value ? baseColor : baseColor + "33"; // 33 = 20% opacity
    ctx.beginPath();
    for (let i = 0; i < points.length; i++) {
      const [x, y] = points[i];
      if (i === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    }
    ctx.stroke();

    for (const [x, y] of points) {
      ctx.beginPath();
      ctx.arc(x, y, pointRadius, 0, Math.PI * 2);
      ctx.fillStyle =
        key === settingSelect.value ? baseColor : baseColor + "33";
      ctx.fill();
    }
  });
  let playheadX = getPlayhead();

  // Draw the playhead
  ctx.strokeStyle = "green";
  ctx.lineWidth = 2;
  ctx.beginPath();
  ctx.moveTo(playheadX, 0);
  ctx.lineTo(playheadX, canvas.height);
  ctx.stroke();
}

drawAll();
