import { Box } from "@mui/material";

export default function Viewport() {
  return (
    <Box width="100%" height="100%">
      <canvas
        id="viewport-canvas"
        style={{
          outline: "none",
        }}
        onContextMenu={(e) => e.preventDefault()}
      ></canvas>
    </Box>
  );
}
