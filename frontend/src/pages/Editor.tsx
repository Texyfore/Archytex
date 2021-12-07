import { Box } from "@mui/material";
import React from "react";
import EditorMenu from "../components/viewport-components/EditorMenu";
import EditorAppBar from "../components/viewport-components/EditorAppBar";
import AppBarOffset from "../components/AppBarOffset";
import Viewport from "../components/viewport-components/Viewport";

const appBarHeight = 48;

export default function Editor() {
  return (
    <React.Fragment>
      <EditorAppBar />
      <AppBarOffset variant='dense' />
      <Box display='flex' height={`calc(100vh - ${appBarHeight}px)`}>
        <Viewport />
        <EditorMenu />
      </Box>
    </React.Fragment>
  );
}
