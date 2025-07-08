import { wasmInterface } from "../wasm.js";
import { setBlackPoints } from "../visualizerCanvas/visualizer.js";
/// Uploads can be either audio or image files. Depending on the file type,
/// we process the upload and send it to the backend.
const input = document.getElementById("upload");

// Handle upload event.
input.addEventListener("change", (event) => {
  console.log("file uploaded");
  const file = event.target.files[0];
  if (!file) return;
  const type = file.type;

  if (type.startsWith("audio/")) {
    audioToBackend(file);
  } else if (type.startsWith("image/")) {
    imageToBackend(file);
  } else {
    alert("Unsupported file type:", type);
  }
});

// Send audio to backend.
export async function audioToBackend(file) {
  const arrayBuffer = await file.arrayBuffer();
  // Convert to Uint8Array
  const uint8Array = new Uint8Array(arrayBuffer);
  // Send to backend
  try {
    await wasmInterface.audio_to_backend(uint8Array);
    await wasmInterface.process_audio_to_coords();
    await wasmInterface.process_coords_to_audio();
  } catch (e) {
    console.error("Error uploading image", e);
  }
}

// Send image to backend
export async function imageToBackend(file) {
  try {
    const arrayBuffer = await file.arrayBuffer();
    const uint8Array = new Uint8Array(arrayBuffer);
    await wasmInterface.image_to_backend(uint8Array);
    await wasmInterface.process_image_to_coords();
    await wasmInterface.process_image_to_black_coords();
    await wasmInterface.process_coords_to_audio();
    const coordsArrayBuffer = await wasmInterface.get_black_coords();
    console.log("coordsArrayBuffer: {}", coordsArrayBuffer);
    setBlackPoints(coordsArrayBuffer);
  } catch (e) {
    console.error("Error uploading image:", e);
  }
}
