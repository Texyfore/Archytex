import init, { Callback, Channel, Resources, run } from "./pkg/web_runner.js";

document.oncontextmenu = () => { return false };

let rightDown = false;

init().then(() => {
    const channel = new Channel();
    const callback = new Callback();
    const resources = new Resources();
    const sender = channel.sender();

    // document.addEventListener("mousedown", ev => {
    //     if (ev.button === 2) {
    //         document.getElementById("viewport-canvas").requestPointerLock();
    //         rightDown = true;
    //     }
    // });

    // document.addEventListener("mouseup", ev => {
    //     if (ev.button == 2) {
    //         document.exitPointerLock();
    //         rightDown = false;
    //     }
    // })

    // document.addEventListener("mousemove", ev => {
    //     if (rightDown) {
    //         sender.movement(ev.movementX, ev.movementY);
    //     }
    // });


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