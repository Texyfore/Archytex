import React, { useState } from "react";

import { Project } from "../../../../services/projects";

import RenderCardGrid from "./RenderGrid";
import ProjectActionsMenu from "./ProjectActionsMenu";
import ProjectRow from "./ProjectRow";

interface Props {
  project: Project;
}

export default function ProjectListItem({ project }: Props) {
  //Project collapse
  const [openProject, setOpenProject] = React.useState(false);
  const handleProjectClick = () => {
    setOpenProject(!openProject);
  };

  //Edit project menu
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);
  const ProjectActionsMenuOpen = Boolean(anchorEl);
  const handleProjectActionsMenuClick = (
    event: React.MouseEvent<HTMLButtonElement>
  ) => {
    setAnchorEl(event.currentTarget);
  };
  const handleProjectActionsMenuClose = () => {
    setAnchorEl(null);
  };

  return (
    <>
      <ProjectRow
        project={project}
        openProject={openProject}
        handleProjectClick={handleProjectClick}
        handleProjectActionsMenuClick={handleProjectActionsMenuClick}
      />

      <RenderCardGrid project={project} open={openProject} />

      <ProjectActionsMenu
        project={project}
        anchorEl={anchorEl}
        open={ProjectActionsMenuOpen}
        handleClose={handleProjectActionsMenuClose}
      />
    </>
  );
}
