import React from "react";
import {
  Box,
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogContentText,
  DialogTitle,
  IconButton,
  List,
  ListItemButton,
  Paper,
  PaperProps,
} from "@mui/material";
import Draggable from "react-draggable";
import { Close } from "@mui/icons-material";
import TextureLibrary from "./TextureLibrary";

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

interface LibraryDialogProps {
  open: boolean;
  handleClose: () => void;
}

export default function LibraryDialog({
  open,
  handleClose,
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
      <DialogTitle style={{ cursor: "move" }} id='draggable-dialog-title'>
        Texture library
      </DialogTitle>
      <DialogContent>
        <Box width={550}>
          <TextureLibrary />
        </Box>
      </DialogContent>
      <DialogActions>
        <Button onClick={handleClose}>Import</Button>
      </DialogActions>
    </Dialog>
  );
}
