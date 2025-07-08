let source = null;

let speed = 1.0;

export function createAudioGraph(audioContext, buffer) {
  source = audioContext.createBufferSource();
  const splitter = audioContext.createChannelSplitter(2);
  const analyserL = audioContext.createAnalyser();
  const analyserR = audioContext.createAnalyser();

  // Configure source
  source.buffer = buffer;

  // Add visual analysers
  source.connect(splitter);
  source.connect(audioContext.destination); // Output to speakers

  splitter.connect(analyserL, 0); // Left channel → analyserL
  splitter.connect(analyserR, 1); // Right channel → analyserR

  source.playbackRate.value = speed;

  return { source, analyserL, analyserR };
}
