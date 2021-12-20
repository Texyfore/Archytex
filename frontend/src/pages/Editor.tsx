import { Box } from "@mui/material";
import React, { useEffect } from "react";
import EditorMenu from "../components/editor-components/EditorMenu";
import EditorAppBar from "../components/editor-components/EditorAppBar";
import AppBarOffset from "../components/AppBarOffset";
import Viewport from "../components/editor-components/Viewport";
import EditorHandle from "../EditorUtils";

const appBarHeight = 48;

export default function Editor() {
  useEffect(() => {}, []);

  return (
    <React.Fragment>
      <EditorAppBar />
      <AppBarOffset variant="dense" />
      <Box display="flex" height={`calc(100vh - ${appBarHeight}px)`}>
        <Viewport />
        <EditorMenu />
      </Box>
    </React.Fragment>
  );
}
