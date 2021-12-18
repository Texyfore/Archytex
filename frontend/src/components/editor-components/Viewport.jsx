import { Box } from "@mui/material";
import { useEffect } from "react";
import useDimensions from "react-cool-dimensions";

let editor = undefined;
let editorInitialized = false;
let currentCanvasSize = [1024, 768];
let desiredCanvasSize = undefined;

export default function Editor() {
  useEffect(() => {
    import("viewport").then((module) => {
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

        if (currentCanvasSize !== desiredCanvasSize) {
          let buffer = new Uint16Array(desiredCanvasSize);
          let bytes = new Uint8Array(buffer.buffer);
          let packet = new Uint8Array([
            0,
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
          ]);
          editor.sendPacket(packet);
          currentCanvasSize = desiredCanvasSize;
        }
      }
    }, 100);
  }, []);

  const { observe } = useDimensions({
    onResize: ({ width, height }) => {
      desiredCanvasSize = [width, height];
    },
  });

  return (
    <Box
      width='100%'
      height='100%'
      ref={observe}
    >
      <canvas
        id='viewport-canvas'
        style={{
          outline: "none",
        }}
        onContextMenu={(e) => e.preventDefault()}
      ></canvas>
    </Box>
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
