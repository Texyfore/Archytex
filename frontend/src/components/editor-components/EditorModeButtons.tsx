import { Chair } from "@mui/icons-material";
import { Box, ToggleButton, ToggleButtonGroup, Tooltip } from "@mui/material";
import React from "react";
import { useTranslation } from "react-i18next";
import FaceSelectIcon from "./icons/FaceSelectIcon";
import MeshSelectIcon from "./icons/MeshSelectIcon";
import VertexSelectIcon from "./icons/VertexSelectIcon";

type EditorMode = "solid" | "face" | "vertex" | "prop";
interface EditorModeButtonsProps {
  editorMode: EditorMode;
  handleEditorModeChange: (e: any) => void;
}

export default function EditorModeButtons({
  editorMode,
  handleEditorModeChange,
}: EditorModeButtonsProps) {
  const { t } = useTranslation();
  const meshTooltipText = t("mesh_select_mode");
  const propTooltipText = t("prop_mode");
  const faceTooltipText = t("face_select_mode");
  const vertexTooltipText = t("vertex_select_mode");
  return (
    <Box position='absolute' top={58} left={220}>
      <ToggleButtonGroup
        value={editorMode}
        exclusive
        onChange={handleEditorModeChange}
        color='primary'
        size='small'
        sx={{ height: 30.75 }}
      >
        <ToggleButton value='prop'>
          <Tooltip title={propTooltipText}>
            <Box marginTop={0.8}>
              <Chair />
            </Box>
          </Tooltip>
        </ToggleButton>
        <ToggleButton value='solid'>
          <Tooltip title={meshTooltipText}>
            <Box marginTop={0.8}>
              <MeshSelectIcon />
            </Box>
          </Tooltip>
        </ToggleButton>
        <ToggleButton value='face'>
          <Tooltip title={faceTooltipText}>
            <Box marginTop={0.8}>
              <FaceSelectIcon />
            </Box>
          </Tooltip>
        </ToggleButton>
        <ToggleButton value='vertex'>
          <Tooltip title={vertexTooltipText}>
            <Box marginTop={0.8}>
              <VertexSelectIcon />
            </Box>
          </Tooltip>
        </ToggleButton>
      </ToggleButtonGroup>
    </Box>
  );
}
