import { Box } from "@mui/material";
import React from "react";

export default function Editor() {
  return (
    <Box width='100%' height='100%' sx={{ backgroundColor: "GrayText" }}>
      <script type='module' src='web/glue.js'></script>
    </Box>
  );
}
