import React from "react";

import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Box from "@mui/material/Box";

import AppBarEndContent from "./AppBarEndContent";
import AppBarStartContent from "./AppBarStartContent";
import { AppBarOffset } from "./AppBarOffset";

export default function ArchytexAppBar() {
  return (
    <>
      <AppBar position='fixed' color='inherit' elevation={0}>
        <Toolbar>
          <Box width='100%' display='flex' justifyContent='space-between'>
            <AppBarStartContent />
            <AppBarEndContent />
          </Box>
        </Toolbar>
      </AppBar>
      <AppBarOffset />
    </>
  );
}
