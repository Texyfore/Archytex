import { Box } from "@mui/material";
import React, { useEffect } from "react";
import EditorMenu from "../components/editor-components/EditorMenu";
import EditorAppBar from "../components/editor-components/EditorAppBar";
import AppBarOffset from "../components/AppBarOffset";
import EditorHandle from "../EditorUtils";
import useDimensions from "react-cool-dimensions";

const appBarHeight = 48;
let editorHandle: EditorHandle;

export default function Editor() {
  const { observe } = useDimensions({
    onResize: ({ width, height }) => {
      editorHandle.setResolution(width, height);
    },
  });

  useEffect(() => {
    editorHandle = new EditorHandle();
  }, []);

  return (
    <React.Fragment>
      <EditorAppBar />
      <AppBarOffset variant="dense" />
      <Box display="flex" height={`calc(100vh - ${appBarHeight}px)`} overflow="hidden">
        <Box width='100%' height="100%" ref={observe} sx={{backgroundColor: '#0c0c0c'}}></Box>
        <EditorMenu />
      </Box>
      <canvas id="viewport-canvas" style={{position: 'absolute', top: `${appBarHeight}px`}}></canvas>
    </React.Fragment>
  );
}
