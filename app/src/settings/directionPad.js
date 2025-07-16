import { wasmInterface } from "../wasm.js";

const directionPad = document.getElementById("directionPad");
const selectedDirections = new Set();

// Map of allowed directions per method
const directionMap = {
  full: [],
  outline: [],
  scan: [1, 5],
  snake: [1, 3, 5, 7],
  dynamic: [1, 2, 3, 4, 5, 6, 7, 8],
};

// Show/hide buttons based on selected method
export function updateDirectionPad(method) {
  const allowed = directionMap[method] || [];
  const buttons = directionPad.querySelectorAll("button");

  buttons.forEach((btn) => {
    const dir = btn.getAttribute("data-dir");
    if (dir === "center") {
      btn.style.display = "inline-block"; // always show the center/start
    } else if (allowed.includes(parseInt(dir))) {
      btn.style.display = "inline-block";
    } else {
      btn.style.display = "none";
    }
  });

  // Just reset local state — DO NOT call WASM here
  selectedDirections.clear();
  directionPad
    .querySelectorAll("button")
    .forEach((btn) => btn.classList.remove("active"));
}

directionPad.addEventListener("click", (event) => {
  const button = event.target.closest("button");
  if (!button) return;

  const directionAttr = button.getAttribute("data-dir");
  if (!directionAttr || directionAttr === "center") return;

  const value = parseInt(directionAttr, 10);
  if (isNaN(value)) return;

  // Toggle active class and selection
  if (selectedDirections.has(value)) {
    selectedDirections.delete(value);
    button.classList.remove("active");
  } else {
    selectedDirections.add(value);
    button.classList.add("active");
  }

  // Convert to array and send to backend
  const directionArray = Array.from(selectedDirections);
  if (directionArray != []) {
    wasmInterface.set_directions(directionArray);
  } else {
    wasmInterface.reset_directions();
  }
});
