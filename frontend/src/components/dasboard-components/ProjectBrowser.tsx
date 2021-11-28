import React, { useState } from "react";
import { Box, Button, IconButton, List, Tooltip } from "@mui/material";
import { LibraryAdd } from "@mui/icons-material";
import { styled, useTheme } from "@mui/material/styles";
import SearchBar from "../SearchBar";
import ProjectNewModal from "./ProjectNewModal";
import { Project, useProjects } from "../../services/projects";
import ProjectRow from "./ProjectRow";

const headerHeight = 50;
const projectMenuHeight = 60;
const ProjectList = styled(List)(({ theme }) => ({
  border: "none",
  overflowY: "scroll",
  height: `calc(100vh - 56px - ${headerHeight + projectMenuHeight}px)`,
  [`${theme.breakpoints.up("xs")} and (orientation: landscape)`]: {
    height: `calc(100vh - 48px - ${headerHeight + projectMenuHeight}px)`,
  },
  [theme.breakpoints.up("sm")]: {
    height: `calc(100vh - 64px - ${headerHeight + projectMenuHeight}px)`,
  },
  [theme.breakpoints.up("lg")]: {
    height: `calc(100% - ${headerHeight + projectMenuHeight}px)`,
  },
}));

export default function ProjectBrowser() {
  //new project modal
  const [newProjectModalOpen, setNewProjectModalOpen] = useState(false);
  const handleNewProjectModalOpen = () => setNewProjectModalOpen(true);
  const handleNewProjectModalClose = () => setNewProjectModalOpen(false);
  const { state: projects } = useProjects();
  return (
    <React.Fragment>
      {/* Project browser actions */}
      <Box
        height={projectMenuHeight}
        display='flex'
        justifyContent='space-between'
        paddingX={{ xs: 2, sm: 4 }}
        borderBottom={`1px solid ${
          useTheme().palette.mode === "dark" ? "#1F1F1F" : "#EBE7EC"
        }`}
      >
        <Box display={{ xs: "none", md: "block" }}>
          <Button
            size='large'
            color='inherit'
            startIcon={<LibraryAdd />}
            onClick={handleNewProjectModalOpen}
          >
            New project
          </Button>
        </Box>
        <Box display={{ xs: "block", md: "none" }}>
          <Tooltip title='Create new project'>
            <IconButton size='large' onClick={handleNewProjectModalOpen}>
              <LibraryAdd />
            </IconButton>
          </Tooltip>
        </Box>
        <Box>
          <SearchBar />
        </Box>
      </Box>

      {/* Project list */}
      <ProjectList>
        {projects.projects.map((project: Project) => (
          <ProjectRow project={project} />
        ))}
      </ProjectList>

      <ProjectNewModal
        handleModalClose={handleNewProjectModalClose}
        handleModalOpen={handleNewProjectModalOpen}
        modalOpen={newProjectModalOpen}
      ></ProjectNewModal>
    </React.Fragment>
  );
}
