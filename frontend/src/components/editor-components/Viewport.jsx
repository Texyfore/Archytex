import { useEffect } from "react";
import useDimensions from "react-cool-dimensions";

let editor = undefined;
let editorInitialized = false;

export default function Editor() {
  useEffect(() => {
    import("viewport").then(module => {
      editor = module;
      module.main();
    });

    // Packet loop
    setInterval(() => {
      if (editor !== undefined) {
        let packet = editor.queryPacket();
        if (packet !== undefined) {
          onPacket(packet);
        }
      }
    }, 100);
  });

  const { observe } = useDimensions({
    onResize: ({ width, height }) => {
      console.log(`[${width}x${height}]`);
    }
  });

  return (
    <canvas
      id="viewport-canvas"
      style={{
        backgroundColor: "red",
        width: "80%",
        height: "100%",
        outline: "none",
      }}
      ref={observe}
      onContextMenu={(e) => e.preventDefault()}
    ></canvas>
  );
}

function onPacket(packet) {
  switch (packet[0]) {
    case 0:
      console.log("Editor has finished initialization");
      editorInitialized = true;
      break;
    default:
      console.warn("Unexpected packet from editor");
      break;
  }
}