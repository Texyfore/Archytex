import { Box } from "@mui/material";
import React from "react";
import EditorMenu from "../components/editor-components/EditorMenu";
import EditorAppBar from "../components/editor-components/EditorAppBar";
import AppBarOffset from "../components/AppBarOffset";
import Viewport from "../components/editor-components/Viewport.jsx";

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
