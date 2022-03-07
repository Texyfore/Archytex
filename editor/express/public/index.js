import init, { Callback, Channel, run } from "./pkg/web_runner.js";

const canvas = document.getElementById("viewport-canvas");
let rightDown = false;

document.oncontextmenu = () => { return false };

init().then(() => {
    const channel = new Channel();
    const callback = new Callback((scene) => {
        console.log(`[wasm] saved scene (${scene.length})`);
    }, (button) => {
        console.log(`[wasm] button feedback ${button}`);
    });

    const sender = channel.sender();

    canvas.addEventListener("mousedown", ev => {
        if (ev.button === 2) {
            canvas.requestPointerLock();
            sender.setPointerLock(true);
            rightDown = true;
        }
    });

    canvas.addEventListener("mouseup", ev => {
        if (ev.button == 2) {
            document.exitPointerLock();
            sender.setPointerLock(false);
            rightDown = false;
        }
    })

    canvas.addEventListener("mousemove", ev => {
        if (rightDown) {
            sender.movement(ev.movementX, ev.movementY);
        }
    });

    run(channel, callback);
});