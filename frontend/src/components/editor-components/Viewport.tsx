import React, { useEffect } from "react";
import { Box } from "@mui/material";

import init from "../../wasm/viewport.js";

export function handleMessage(msg: any) {
  alert(msg);
}

export default function Editor() {
  useEffect(() => {
    init("viewport_bg.wasm");
  }, []);

  return (
    <canvas
      id='viewport-canvas'
      style={{ backgroundColor: "red", width: "80%", height: "100%" }}
    ></canvas>
  );
}
