import React, { useState } from "react";
import {
  Box,
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  IconButton,
  Paper,
  PaperProps,
  Tooltip,
  Typography,
} from "@mui/material";
import Draggable from "react-draggable";
import { Close, FilterList } from "@mui/icons-material";
import TextureLibrary from "./TextureLibrary";
import SearchBar from "../../SearchBar";

function PaperComponent(props: PaperProps) {
  return (
    <Draggable
      handle='#draggable-dialog-title'
      cancel={'[class*="MuiDialogContent-root"]'}
    >
      <Paper {...props} />
    </Draggable>
  );
}

type LibraryType = "textureLibrary" | "propLibrary" | "projectLibrary";

interface LibraryDialogProps {
  open: boolean;
  handleClose: () => void;
  libraryType: LibraryType;
}

export default function LibraryDialog({
  open,
  handleClose,
  libraryType,
}: LibraryDialogProps) {
  const descriptionElementRef = React.useRef<HTMLElement>(null);
  React.useEffect(() => {
    if (open) {
      const { current: descriptionElement } = descriptionElementRef;
      if (descriptionElement !== null) {
        descriptionElement.focus();
      }
    }
  }, [open]);

  return (
    <Dialog
      open={open}
      onClose={handleClose}
      scroll='paper'
      PaperComponent={PaperComponent}
      aria-labelledby='draggable-dialog-title'
    >
      <IconButton
        onClick={handleClose}
        sx={{
          position: "absolute",
          right: 8,
          top: 8,
          color: (theme) => theme.palette.grey[500],
        }}
      >
        <Close />
      </IconButton>
      <DialogTitle
        style={{ cursor: "move", borderBottom: "1px solid grayText" }}
        id='draggable-dialog-title'
      >
        <Box display='flex' flexWrap='wrap' marginBottom={2}>
          <Typography variant='h6'>
            {libraryType === "textureLibrary"
              ? "Texture library"
              : libraryType === "propLibrary"
              ? "Prop library"
              : libraryType === "projectLibrary"
              ? "Project library"
              : "Library"}
          </Typography>
        </Box>
        <Box
          sx={{
            position: "absolute",
            right: 60,
            top: 10,
          }}
        >
          <Tooltip title='Filter results'>
            <Button endIcon={<FilterList />} color='inherit'>
              Filter results
            </Button>
          </Tooltip>
        </Box>
        <SearchBar />
      </DialogTitle>
      <DialogContent>
        <Box width={550}>
          <TextureLibrary />
        </Box>
      </DialogContent>
      <DialogActions>
        <Button onClick={handleClose}>Accept</Button>
      </DialogActions>
    </Dialog>
  );
}
