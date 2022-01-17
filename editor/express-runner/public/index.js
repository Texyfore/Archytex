import init, { Channel, run } from "./pkg/web_runner.js";

document.oncontextmenu = () => { return false };

init().then(() => {
  const channel = new Channel();
  const wasmEndpoint = channel.wasmEndpoint(onLog, onFatalError);
  run(wasmEndpoint);
});

function onLog(log) {
  console.log(`[wasm] ${log}`);
}

function onFatalError(error) {
  console.error(`[wasm] ${error}`);
}