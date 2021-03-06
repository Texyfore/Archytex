import React from "react";
import { useLocation } from "react-router-dom";

import useScrollTrigger from "@mui/material/useScrollTrigger";
import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Box from "@mui/material/Box";

import AppBarEndContent from "./AppBarEndContent";
import AppBarStartContent from "./AppBarStartContent";

interface Props {
  window?: () => Window;
  children: JSX.Element;
}

function ElevationScroll(props: Props) {
  const { children, window } = props;
  const trigger = useScrollTrigger({
    disableHysteresis: true,
    threshold: 0,
    target: window ? window() : undefined,
  });

  return React.cloneElement(children, {
    elevation: trigger ? 1 : 0,
    color: trigger ? "inherit" : "transparent",
  });
}

export default function ArchytexAppBar() {
  const location = useLocation();

  return !location.pathname.includes("editor") ? (
    <ElevationScroll>
      <AppBar position='fixed' color='transparent'>
        <Toolbar>
          <Box width='100%' display='flex' justifyContent='space-between'>
            <AppBarStartContent />
            <AppBarEndContent />
          </Box>
        </Toolbar>
      </AppBar>
    </ElevationScroll>
  ) : (
    <></>
  );
}
