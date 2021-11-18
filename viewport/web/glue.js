import init, {sendMessage} from "./app/viewport.js";

addEventListener("load", () => {
    init();
});
addEventListener("contextmenu", e => e.preventDefault());

window.asd = () => {
    sendMessage('{"Dummy": "Hello, World!"}');
}

export function handleMessage(msg) {
    alert(msg);
}