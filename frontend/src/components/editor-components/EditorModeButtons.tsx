import React from "react";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import ToggleButtonGroup from "@mui/material/ToggleButtonGroup";
import ToggleButton from "@mui/material/ToggleButton";
import Tooltip from "@mui/material/Tooltip";

import { Chair } from "@mui/icons-material";

import FaceSelectIcon from "./icons/FaceSelectIcon";
import MeshSelectIcon from "./icons/MeshSelectIcon";
import VertexSelectIcon from "./icons/VertexSelectIcon";
import FaceSelectIconBlue from "./icons/FaceSelectIconBlue";
import MeshSelectIconBlue from "./icons/MeshSelectIconBlue";
import VertexSelectIconBlue from "./icons/VertexSelectIconBlue";

type EditorMode = "solid" | "face" | "vertex" | "prop";
interface EditorModeButtonsProps {
  editorMode: EditorMode;
  handleEditorModeChange: (mode: EditorMode, send: boolean) => void;
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

  const handleChange = (e: any, value: EditorMode) => {
    if (e !== null) {
      console.log(value);
      handleEditorModeChange(value, true);
    }
  };

  return (
    <Box position='absolute' top={58} left={95}>
      <ToggleButtonGroup
        value={editorMode.toString()}
        exclusive
        onChange={handleChange}
        color='primary'
        size='small'
        sx={{ height: 30.75 }}
      >
        <Tooltip title={meshTooltipText}>
          <ToggleButton
            value='solid'
            sx={
              editorMode === "solid"
                ? { backgroundColor: "rgba(57, 159, 237, 0.25)" }
                : {}
            }
          >
            {editorMode === "solid" ? (
              <MeshSelectIconBlue />
            ) : (
              <MeshSelectIcon />
            )}
          </ToggleButton>
        </Tooltip>
        <Tooltip title={faceTooltipText}>
          <ToggleButton
            value='face'
            sx={
              editorMode === "face"
                ? { backgroundColor: "rgba(57, 159, 237, 0.25)" }
                : {}
            }
          >
            {editorMode === "face" ? (
              <FaceSelectIconBlue />
            ) : (
              <FaceSelectIcon />
            )}
          </ToggleButton>
        </Tooltip>
        <Tooltip title={vertexTooltipText}>
          <ToggleButton
            value='vertex'
            sx={
              editorMode === "vertex"
                ? { backgroundColor: "rgba(57, 159, 237, 0.25)" }
                : {}
            }
          >
            {editorMode === "vertex" ? (
              <VertexSelectIconBlue />
            ) : (
              <VertexSelectIcon />
            )}
          </ToggleButton>
        </Tooltip>
        <ToggleButton value='prop'>
          <Tooltip title={propTooltipText}>
            <Chair />
          </Tooltip>
        </ToggleButton>
      </ToggleButtonGroup>
    </Box>
  );
}
