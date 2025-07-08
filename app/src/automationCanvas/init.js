// init.js
import { drawAll } from "./canvas.js";
import "./automation.js";
import "./constants.js";
import "./events.js";
import "./playhead.js";

let interval = 50; // ms
while (true) {
  drawAll();
  await new Promise((r) => setTimeout(r, interval));
}
