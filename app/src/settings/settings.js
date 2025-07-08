export const settings = {
  mode: "full",
  intAmount: 10,
  threshold: 50,
  pixThreshold: 50,
  sampleRate: 44100,
  edgeDetecion: true,
  size: 300,
  repeat: 1,

  loopAudio: true,
  playbackRate: 1,
  dotMode: true,
  scale: 300,
  stroke: 1,
  persistence: 0.05,
  hue: 140,
  lineColor: "",
  imageOpacity: 0,
  noise: 0,
  centerX: 300,
  centerY: 300,
  clipLength: 10,
};

export function updateColor() {
  settings.lineColor = `hsl(${settings.hue}, 100%, 40%)`;
}
