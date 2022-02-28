import init, { Callback, Channel, Resources, run } from "./pkg/web_runner.js";

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
    const resources = new Resources();
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


    Promise.all([
        resource("/assets/bricks.png"),
        resource("/assets/table.amdl"),
    ]).then(([bricks, table]) => {
        resources.addTexture(2, bricks);
        resources.addProp(0, table);
        run(channel, callback, resources);
    });
});

async function resource(url) {
    const res = await fetch(url);
    const buf = await res.arrayBuffer();
    return new Uint8Array(buf);
}