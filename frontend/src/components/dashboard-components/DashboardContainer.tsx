import React from "react";

import Box from "@mui/material/Box";

import { ColorMode, useColorMode } from "../../services/colorMode";

interface Props {
  children: null | JSX.Element | JSX.Element[];
}
export default function DashboardContainer({ children }: Props) {
  const [colorMode, _] = useColorMode();

  return (
    <Box
      width='100%'
      height='calc(100vh - 64px)'
      borderTop={
        colorMode === ColorMode.Dark ? "1px solid #2E2E2E" : "1px solid #BABABA"
      }
      display='flex'
      flexWrap='nowrap'
    >
      {children}
    </Box>
  );
}
