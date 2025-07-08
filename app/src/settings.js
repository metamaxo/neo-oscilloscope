export const canvas = document.getElementById("oscilloscope");

// style settings
export const settings = {
  style: {
    loop: true,
    drawMode: "dots",
    scale: 300,
    stroke: 1,
    persistence: 0.05,
    hue: 140,
    lineColor: "",
    imageOpacity: 0,
    noise: 0,
  },
  processing: {
    threshold: 50,
    pixThreshold: 50,
    repeat: 1,
    intAmount: 20,
  },
  effects: {
    gain: 4,
    freq: 1,
    lp: 20,
    pan: 0,
  },
  automation: {
    clipLength: 10,
  },
};

// update settings
export function updateSetting(type, key, value) {
  console.log("updating setting {} to {}", key, value);
  settings[type][key] = value;
}

export function updateColor() {
  settings.style.lineColor = `hsl(${settings.style.hue}, 100%, 40%)`;
}
