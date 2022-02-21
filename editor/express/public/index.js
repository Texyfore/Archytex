import init, { Callback, Channel, run } from "./pkg/web_runner.js";

document.oncontextmenu = () => { return false };

init().then(() => {
    let channel = new Channel();
    let callback = new Callback();
    let sender = channel.sender();
    run(channel, callback);
});