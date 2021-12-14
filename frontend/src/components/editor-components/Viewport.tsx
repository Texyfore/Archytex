import { useEffect } from "react";

export default function Editor() {
  useEffect(() => {
    import("viewport").then(module => module.main());
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
      onContextMenu={(e) => e.preventDefault()}
    ></canvas>
  );
}
