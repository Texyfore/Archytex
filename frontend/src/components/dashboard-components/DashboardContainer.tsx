import React from "react";

import { styled } from "@mui/material/styles";

import Box from "@mui/material/Box";

import { ColorMode, useColorMode } from "../../services/colorMode";

const Container = styled(Box)(({ theme }) => ({
  height: `calc(100vh - 64px)`,
  [`${theme.breakpoints.up("xs")}`]: {
    height: `calc(100vh - 64px)`,
  },
  [`${theme.breakpoints.up("xs")} and (orientation: landscape)`]: {
    height: `calc(100vh - 54px)`,
  },
  [theme.breakpoints.up("sm")]: {
    height: `calc(100vh - 64px)`,
  },
}));

interface Props {
  children: null | JSX.Element | JSX.Element[];
}
export default function DashboardContainer({ children }: Props) {
  const [colorMode, _] = useColorMode();

  return (
    <Container
      borderTop={
        colorMode === ColorMode.Dark ? "1px solid #2E2E2E" : "1px solid #BABABA"
      }
      display='flex'
      flexWrap='nowrap'
      overflow='scoll'
    >
      {children}
    </Container>
  );
}
