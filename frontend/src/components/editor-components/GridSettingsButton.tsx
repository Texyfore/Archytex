import React from "react";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import Tooltip from "@mui/material/Tooltip";
import IconButton from "@mui/material/IconButton";

import { Grid3x3Rounded } from "@mui/icons-material";

import GridSettingsMenu from "./GridSettingsMenu";

interface GridSettingsButtonProps {
  gridStep: number;
  handleGridStepChange: (e: any) => void;
}

export default function GridSettingsButton({
  gridStep,
  handleGridStepChange,
}: GridSettingsButtonProps) {
  const { t } = useTranslation();
  const gridSettingsTooltipText = t("grid_settings");

  const [gridAnchorEl, setGridAnchorEl] = React.useState<null | HTMLElement>(
    null
  );
  const gridMenuOpen = Boolean(gridAnchorEl);
  const handleGridMenuClick = (event: React.MouseEvent<HTMLElement>) => {
    setGridAnchorEl(event.currentTarget);
  };
  const handleGridMenuClose = () => {
    setGridAnchorEl(null);
  };

  return (
    <>
      <Box position='absolute' top={58} left='calc(100% - 450px)'>
        <Tooltip title={gridSettingsTooltipText}>
          <IconButton onClick={handleGridMenuClick}>
            <Grid3x3Rounded />
          </IconButton>
        </Tooltip>
      </Box>

      <GridSettingsMenu
        gridStep={gridStep}
        handleGridStepChange={handleGridStepChange}
        gridAnchorEl={gridAnchorEl}
        gridMenuOpen={gridMenuOpen}
        handleGridMenuClose={handleGridMenuClose}
      />
    </>
  );
}
