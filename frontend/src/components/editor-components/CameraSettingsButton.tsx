import React from "react";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import Tooltip from "@mui/material/Tooltip";
import IconButton from "@mui/material/IconButton";

import { VideoCameraBack } from "@mui/icons-material";

import CameraSettingsMenu from "./CameraSettingsMenu";

interface CameraSettingsButtonProps {
  cameraSpeed: number;
  handleCameraSpeedChange: (e: any) => void;
}

export default function CameraSettingsButton({
  cameraSpeed,
  handleCameraSpeedChange,
}: CameraSettingsButtonProps) {
  const { t } = useTranslation();
  const cameraSettingsTooltipText = t("camera_settings");

  //Camera settings
  const [cameraAnchorEl, setCameraAnchorEl] =
    React.useState<null | HTMLElement>(null);
  const cameraMenuOpen = Boolean(cameraAnchorEl);
  const handleCameraMenuClick = (event: React.MouseEvent<HTMLElement>) => {
    setCameraAnchorEl(event.currentTarget);
  };
  const handleCameraMenuClose = () => {
    setCameraAnchorEl(null);
  };

  return (
    <>
      <Box position='absolute' top={58} left='calc(100% - 400px)'>
        <Tooltip title={cameraSettingsTooltipText}>
          <IconButton onClick={handleCameraMenuClick}>
            <VideoCameraBack />
          </IconButton>
        </Tooltip>
      </Box>

      <CameraSettingsMenu
        cameraSpeed={cameraSpeed}
        handleCameraSpeedChange={handleCameraSpeedChange}
        cameraAnchorEl={cameraAnchorEl}
        cameraMenuOpen={cameraMenuOpen}
        handleCameraMenuClose={handleCameraMenuClose}
      />
    </>
  );
}
