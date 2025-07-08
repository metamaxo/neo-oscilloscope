import { togglePlayback } from "./state.js"

export const canvas = document.getElementById("oscilloscope");

export const ctx = canvas.getContext("2d");

canvas.addEventListener("click", () => {
  console.log("canvas clicked")
  togglePlayback()
  
});
