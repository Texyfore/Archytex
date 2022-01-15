import init, { Channel, run } from "./pkg/web_runner.js";

init().then(() => {
  const channel = new Channel();
  const wasmEndpoint = channel.wasmEndpoint(onFatalError);
  run(wasmEndpoint);
});

function onFatalError(error) {
  console.error(`Fatal error: ${error}`);
}