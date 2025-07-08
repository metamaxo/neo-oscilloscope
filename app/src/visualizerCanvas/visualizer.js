import { clearAnimation, getSourceNode, setAnimationId } from "../state.js";
import { ctx, canvas } from "./canvas.js";
import { updateColor, settings } from "../settings/settings.js";
import { wasmInterface } from "../wasm.js";
const blackPointSet = new Set();

export function setBlackPoints(points) {
  blackPointSet.clear();
  const arr = new Float32Array(points);
  for (let i = 0; i < arr.length; i += 2) {
    const x = arr[i];
    const y = arr[i + 1];

    const pixelX = Math.floor(((x + 1) / 2) * canvas.width);
    const pixelY = Math.floor(((y + 1) / 2) * canvas.height);

    blackPointSet.add(`${pixelX}:${pixelY}`);
  }
  console.log("black points loaded");
}

// takes an audio buffer and uses stereo waveform data to render visuals.
export function startVisualization(analyserL, analyserR, dataL, dataR) {
  // Clear any existing animation frame
  clearAnimation();

  draw();

  // Draw function renders one frame and schedules next
  function draw() {
    const currentSource = getSourceNode();
    if (!currentSource) return; // no source, stop drawing

    // Schedule next animation frame and save ID
    const id = requestAnimationFrame(draw);
    setAnimationId(id);

    // Populate data arrays with latest waveform samples
    analyserL.getFloatTimeDomainData(dataL);
    analyserR.getFloatTimeDomainData(dataR);

    // Draw trailing effect
    ctx.fillStyle = `rgba(0, 0, 0, ${1 - settings.persistence / 100})`;
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    // Generate XY points from waveform data
    const points = wasmInterface.generate_points(
      dataL,
      dataR,
      settings.scale,
      settings.centerX,
      settings.centerY,
    );

    if (settings.dotMode) {
      updateColor();
      ctx.fillStyle = settings.lineColor;
      ctx.beginPath();
      for (let i = 0; i < points.length; i += 2) {
        const x = points[i];
        const y = points[i + 1];
        if (!shouldDraw(x, y)) continue;

        ctx.moveTo(x, y);
        ctx.arc(x, y, settings.stroke, 0, Math.PI * 2);
      }
      ctx.fill();
    } else {
      updateColor();
      ctx.strokeStyle = settings.lineColor;
      ctx.lineWidth = settings.stroke;
      ctx.beginPath();
      ctx.moveTo(points[0], points[1]);
      for (let i = 2; i < points.length; i += 2) {
        ctx.lineTo(points[i], points[i + 1]);
      }
      ctx.stroke();
    }
  }
}

function shouldDraw(x, y) {
  const opacity = settings.imageOpacity;

  // Always draw if slider is 0
  if (opacity === 0) return true;

  const key = `${Math.floor(x)}:${Math.floor(y)}`;

  if (opacity === 100) {
    // Fully strict: only draw if in blackPointSet
    return blackPointSet.has(key);
  }

  // Partial filtering
  const shouldFilter = Math.random() * 100 < opacity;

  if (shouldFilter) {
    return blackPointSet.has(key); // filter: only draw if it's in the set
  } else {
    return true; // allow it even if not in set
  }
}

export function stopVisualization() {
  clearAnimation();
}
