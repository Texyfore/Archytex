import React from "react";

import { useTranslation } from "react-i18next";

import Typography from "@mui/material/Typography";
import IconButton from "@mui/material/IconButton";
import ListItem from "@mui/material/ListItem";
import ListItemButton from "@mui/material/ListItemButton";
import ListItemIcon from "@mui/material/ListItemIcon";
import Tooltip from "@mui/material/Tooltip";

import {
  KeyboardArrowDown,
  KeyboardArrowRight,
  MoreVert,
} from "@mui/icons-material";

import { Project } from "../../../services/projects";
import { ColorMode, useColorMode } from "../../../services/colorMode";

interface Props {
  project: Project;
  openProject: boolean;
  handleProjectClick: () => void;
  handleProjectActionsMenuClick: (e: any) => void;
}
export default function ProjectRow({
  project,
  openProject,
  handleProjectClick,
  handleProjectActionsMenuClick,
}: Props) {
  const { t } = useTranslation();
  const tooltipText = t("project_actions");

  const [colorMode, _] = useColorMode();

  return (
    <ListItem
      key={project.id}
      disablePadding
      secondaryAction={
        <Tooltip title={tooltipText}>
          <IconButton onClick={handleProjectActionsMenuClick}>
            <MoreVert />
          </IconButton>
        </Tooltip>
      }
    >
      <ListItemButton
        onClick={handleProjectClick}
        sx={{
          paddingY: 3,
          borderRadius: 2,
          backgroundColor: openProject
            ? colorMode === ColorMode.Dark
              ? "#252524"
              : "#E0E0E0"
            : "initial",
        }}
      >
        <ListItemIcon>
          {openProject ? <KeyboardArrowDown /> : <KeyboardArrowRight />}
        </ListItemIcon>
        <Typography
          noWrap
          variant='h6'
          width={{ xs: "200px", md: "50%", lg: "unset" }}
        >
          {project.title}
        </Typography>
        <Typography variant='caption' marginLeft={2}>
          ( {project.renders.length} {t("renders_count")} )
        </Typography>
      </ListItemButton>
    </ListItem>
  );
}
