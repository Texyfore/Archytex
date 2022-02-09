import React from "react";

import Box from "@mui/material/Box";

import LogoWithText from "../general-components/LogoWithText";
import AppBarNavigationButtons from "./AppBarNavigationButtons";

export default function AppBarStartContent() {
  return (
    <Box display='flex'>
      <LogoWithText />
      <AppBarNavigationButtons />
    </Box>
  );
}
