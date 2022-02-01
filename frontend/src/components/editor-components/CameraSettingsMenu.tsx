import React from "react";
import { Menu, List, ListItem, Box, Typography, Slider } from "@mui/material";
import { useTranslation } from "react-i18next";

interface CameraSettingsMenuProps {
  cameraAnchorEl: Element | ((element: Element) => Element) | null | undefined;
  cameraMenuOpen: boolean;
  handleCameraMenuClose: () => void;
  cameraSpeed: number;
  handleCameraSpeedChange: (e: any) => void;
}

export default function CameraSettingsMenu({
  cameraAnchorEl,
  cameraMenuOpen,
  handleCameraMenuClose,
  cameraSpeed,
  handleCameraSpeedChange,
}: CameraSettingsMenuProps) {
  const { t } = useTranslation();

  return (
    <Menu
      anchorEl={cameraAnchorEl}
      id='camera-menu'
      open={cameraMenuOpen}
      onClose={handleCameraMenuClose}
      PaperProps={{
        elevation: 0,
        sx: {
          overflow: "visible",
          filter: "drop-shadow(0px 2px 8px rgba(0,0,0,0.5))",
          mt: 1.5,
          borderRadius: 2,
          "&:before": {
            content: '""',
            display: "block",
            position: "absolute",
            top: 0,
            right: 25,
            width: 10,
            height: 10,
            bgcolor: "paper.background",
            transform: "translateY(-50%) rotate(45deg)",
            zIndex: 0,
          },
        },
      }}
      transformOrigin={{ horizontal: "right", vertical: "top" }}
      anchorOrigin={{ horizontal: "right", vertical: "bottom" }}
    >
      <List dense>
        <ListItem>
          <Box>
            <Typography gutterBottom>{t("camera_speed")}</Typography>
            <Box width={150}>
              <Slider
                size='small'
                defaultValue={cameraSpeed}
                step={1}
                min={1}
                max={100}
                onChange={handleCameraSpeedChange}
                valueLabelDisplay='auto'
              />
            </Box>
          </Box>
        </ListItem>
      </List>
    </Menu>
  );
}
