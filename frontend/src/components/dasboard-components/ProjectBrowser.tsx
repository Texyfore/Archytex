import React, { useState } from "react";
import {
  Alert,
  AlertColor,
  Box,
  Button,
  CircularProgress,
  IconButton,
  List,
  Snackbar,
  Tooltip,
  Typography,
} from "@mui/material";
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
interface actionFeedbackSnackbarProps {
  text: string;
  severity: AlertColor;
}

export default function ProjectBrowser() {
  //Load projects
  const { state: projects } = useProjects();

  //New project modal
  const [newProjectModalOpen, setNewProjectModalOpen] = useState(false);
  const handleNewProjectModalOpen = () => setNewProjectModalOpen(true);
  const handleNewProjectModalClose = () => setNewProjectModalOpen(false);

  //Action feedback snackbar
  const [actionFeedbackSnackbarOpen, setActionFeedbackSnackbarOpen] =
    useState(false);
  const [actionFeedbackSnackbarParams, setActionFeedbackSnackbarParams] =
    useState<actionFeedbackSnackbarProps>({
      text: "",
      severity: "success",
    });
  const handleActionFeedbackSnackbarClose = () => {
    setActionFeedbackSnackbarOpen(false);
  };
  const handleActionFeedbackSnackbarOpen = (
    text: string,
    severity: AlertColor
  ) => {
    setActionFeedbackSnackbarParams({ text: text, severity: severity });
    setActionFeedbackSnackbarOpen(true);
  };
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
        {projects === undefined ? (
          <Box
            height='100%'
            display='flex'
            justifyContent='center'
            alignItems='center'
            flexDirection='column'
            gap={2}
          >
            <CircularProgress />
            <Typography>Loading projects...</Typography>
          </Box>
        ) : (
          projects.projects.map((project: Project) => (
            <ProjectRow
              key={project.id}
              project={project}
              feedbackSnackbar={handleActionFeedbackSnackbarOpen}
            />
          ))
        )}
      </ProjectList>

      {/* New project modal */}
      <ProjectNewModal
        handleModalClose={handleNewProjectModalClose}
        handleModalOpen={handleNewProjectModalOpen}
        modalOpen={newProjectModalOpen}
        feedbackSnackbar={handleActionFeedbackSnackbarOpen}
      ></ProjectNewModal>

      {/* Action feedback snackbar */}
      <Snackbar
        open={actionFeedbackSnackbarOpen}
        autoHideDuration={4000}
        onClose={handleActionFeedbackSnackbarClose}
        anchorOrigin={{ horizontal: "center", vertical: "bottom" }}
      >
        <Alert
          onClose={handleActionFeedbackSnackbarClose}
          severity={actionFeedbackSnackbarParams.severity}
          sx={{
            width: "100%",
            filter: "drop-shadow(0px 0px 4px rgba(0,0,0,0.5))",
          }}
        >
          {actionFeedbackSnackbarParams.text}
        </Alert>
      </Snackbar>
    </React.Fragment>
  );
}
