import init from "./app/viewport.js";

addEventListener("load", () => {
    init();
});
addEventListener("contextmenu", e => e.preventDefault());

export function handleMessage(msg) {
    alert(msg);
}