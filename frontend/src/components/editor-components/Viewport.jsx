import React, { MouseEventHandler, useEffect } from "react";
import { Box } from "@mui/material";

import * as wasm from "../../wasm/viewport";
import init, {queryPacket} from "../../wasm/viewport";

export default function Editor() {
  useEffect(() => {
    init("viewport_bg.wasm").then((mod) => {
      setInterval(() => {
        console.log(queryPacket());
      }, 500);
    });
  }, []);

  return (
    <canvas
      id="viewport-canvas"
      style={{
        backgroundColor: "red",
        width: "80%",
        height: "100%",
        outline: "none",
      }}
      onContextMenu={(e) => e.preventDefault()}
    ></canvas>
  );
}
