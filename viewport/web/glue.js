import init, {addTexture} from "./app/viewport.js";

addEventListener("load", () => {
    init();
});
addEventListener("contextmenu", e => e.preventDefault());

export function handleMessage(msg) {
    alert(msg);
}

export function downloadAndAddTexture(id, url) {
    fetch(url).then(r => {
        r.blob().then(b => {
            let reader = new FileReader();
            reader.readAsArrayBuffer(b);
            reader.onload = () => {
                addTexture(id, new Uint8Array(reader.result));
            }
        })
    });
}