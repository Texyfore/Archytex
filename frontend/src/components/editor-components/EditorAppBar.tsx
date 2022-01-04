import React from "react";
import { AppBar, Box, Button, Toolbar, Tooltip } from "@mui/material";
import { styled } from "@mui/material/styles";
import ArchytexIcon from "../ArchytexIcon";

const CustomEditorAppBar = styled(AppBar)(({ theme }) => ({
  zIndex: theme.zIndex.drawer + 1,
  backgroundColor: theme.palette.background.paper,
}));

interface EditorAppBarProps {
  onSave: () => void;
}

export default function EditorAppBar({ onSave }: EditorAppBarProps) {
  return (
    <CustomEditorAppBar elevation={0}>
      <Toolbar variant="dense" sx={{ borderBottom: "1px solid #1F1F1F" }}>
        <Box width="100%" height="100%" display="flex" alignItems="center">
          <Box height="100%" display="flex" alignItems="center">
            <Tooltip title="Archytex version 0.0.1" placement="bottom-start">
              <Box>
                <ArchytexIcon size={25} />
              </Box>
            </Tooltip>
          </Box>

          {/* Appbar menu */}
          <Box display="flex">
            <Button
              variant="text"
              color="inherit"
              sx={{ textTransform: "none" }}
              onClick={onSave}
            >
              Save
            </Button>
          </Box>
        </Box>
      </Toolbar>
    </CustomEditorAppBar>
  );
}
