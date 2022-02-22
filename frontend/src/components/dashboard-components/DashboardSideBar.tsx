import React from "react";

import Box from "@mui/material/Box";
import IconButton from "@mui/material/IconButton";

import { PermMedia, PlayArrow, Settings } from "@mui/icons-material";

import { useTranslation } from "react-i18next";

import Tooltip from "@mui/material/Tooltip";

import { ColorMode, useColorMode } from "../../services/colorMode";
import { useSubPage } from "../../services/selectedDashboardSubPage";

export default function DashboardSideBar() {
  const { t } = useTranslation();
  const playTooltipText = t("launch_archytex");
  const projectsTooltipText = t("projects");
  const settingsTooltipText = t("settings");

  const [colorMode, _] = useColorMode();

  const [subPage, setSubPage] = useSubPage();

  return (
    <Box
      width='64px'
      height='100%'
      alignSelf='left'
      borderRight={
        colorMode === ColorMode.Dark ? "1px solid #2E2E2E" : "1px solid #BABABA"
      }
      py={2}
      display={{ xs: "none", md: "flex" }}
      flexDirection='column'
      alignItems='center'
      gap={2}
    >
      <Tooltip title={playTooltipText} placement='right'>
        <IconButton>
          <PlayArrow fontSize='large' />
        </IconButton>
      </Tooltip>
      <Tooltip title={projectsTooltipText} placement='right'>
        <IconButton onClick={() => setSubPage("projects")}>
          <PermMedia fontSize='large' />
        </IconButton>
      </Tooltip>
      <Tooltip title={settingsTooltipText} placement='right'>
        <IconButton onClick={() => setSubPage("settings")}>
          <Settings fontSize='large' />
        </IconButton>
      </Tooltip>
    </Box>
  );
}
