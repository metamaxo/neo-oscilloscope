import { startVisualization, stopVisualization } from "./visualizer.js";
import { settings } from "./settings.js";
import { createAudioGraph } from "./nodes.js";
import { toggleDownload } from "./tools.js";
import { wasmInterface } from "./wasm.js";
import { startPlayhead, resetPlayhead } from "./automationCanvas/playhead.js";

export const audioContext = new AudioContext();
let audioBuffer = null;
let animationId = null;
let sourceNode = null;

export const state = {
  playing: false,
};

export function getAnimationId() {
  return animationId;
}

export function setAnimationId(id) {
  animationId = id;
}

export function setAudioBuffer(buffer) {
  audioBuffer = buffer;
}

export function getAudioBuffer() {
  return audioBuffer;
}

export function clearAnimation() {
  if (animationId) {
    cancelAnimationFrame(animationId);
    animationId = null;
  }
}

export function setSourceNode(source) {
  sourceNode = source;
  toggleDownload(source);
}

export function getSourceNode() {
  return sourceNode;
}

export async function startPlayback() {
  console.log("start playback");

  // Fetch raw WAV audio bytes from backend (Uint8Array)
  let audioBytes;
  try {
    audioBytes = await wasmInterface.get_audio();
  } catch (err) {
    console.error("Failed to fetch audio from backend:", err);
    return;
  }

  // Convert Uint8Array to ArrayBuffer
  const arrayBuffer = audioBytes.buffer.slice(0);

  // Decode WAV bytes into AudioBuffer
  let audioBuffer;
  try {
    audioBuffer = await audioContext.decodeAudioData(arrayBuffer);
  } catch (err) {
    console.error("Error decoding audio:", err);
    return;
  }

  // Stop old source if playing
  if (sourceNode) {
    try {
      sourceNode.stop();
    } catch {}
    sourceNode.disconnect();
  }

  let { source, analyserL, analyserR } = createAudioGraph(
    audioContext,
    audioBuffer,
  );

  setSourceNode(source);
  // Create new AudioBufferSourceNode and assign decoded buffer
  source.loop = true; // Loop playback

  // Connect source to destination (speakers)
  source.connect(audioContext.destination);
  source.start();

  const dataL = new Float32Array(analyserL.fftSize);
  const dataR = new Float32Array(analyserR.fftSize);
  startVisualization(analyserL, analyserR, dataL, dataR);
  startPlayhead();

  state.playing = true;

  setTimeout(() => {
    stopPlayback();
  }, settings.automation.clipLength * 1000);
}

export function stopPlayback() {
  if (sourceNode) {
    try {
      sourceNode.stop();
    } catch {}
    stopVisualization();
    resetPlayhead();
    state.playing = false;
  }
}

export function togglePlayback() {
  console.log("state playing: {}", state.playing);
  if (state.playing) {
    stopPlayback();
  } else {
    startPlayback();
  }
}
