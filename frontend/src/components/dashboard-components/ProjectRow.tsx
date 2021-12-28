import {
  Close,
  Delete,
  Edit,
  InfoOutlined,
  KeyboardArrowDown,
  KeyboardArrowRight,
  MoreVert,
} from "@mui/icons-material";
import {
  Collapse,
  Divider,
  Grid,
  ListItemButton,
  ListItemIcon,
  Menu,
  MenuItem,
  Typography,
  Box,
  Tooltip,
  IconButton,
  ListItem,
  Modal,
  Fade,
  Button,
  Backdrop,
  TextField,
  AlertColor,
  Grow,
  easing,
} from "@mui/material";
import React, { useState } from "react";
import { Project, Render, useProjects } from "../../services/projects";
import RenderCard from "./RenderCard";

interface ProjectRowProps {
  project: Project;
  feedbackSnackbar: (text: string, severity: AlertColor) => void;
}

export default function ProjectRow({
  project,
  feedbackSnackbar,
}: ProjectRowProps) {
  //Read projects
  const { dispatch: dispatchProjects } = useProjects();

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

  //Confirm project delete modal
  const [deleteProjectModalOpen, setDeleteProjectModalOpen] = useState(false);
  const handleDeleteProjectModalOpen = () => {
    setDeleteProjectModalOpen(true);
    handleProjectActionsMenuClose();
  };
  const handleDeleteProjectModalClose = () => setDeleteProjectModalOpen(false);

  //Project delete handling
  //BUG: deleting the last project in the list doesn't make the "Successful deletion" snackbar appear
  const handleProjectDelete = () => {
    dispatchProjects({
      type: "delete",
      id: project.id,
    });
    handleDeleteProjectModalClose();
    feedbackSnackbar("Project deleted successfully!", "success");
  };

  //Title edit handling
  const [underEdit, setUnderEdit] = useState(false);
  const [underEditText, setUnderEditText] = useState("");
  const handleUnderEditStart = () => {
    handleProjectActionsMenuClose();
    setUnderEditText(project.title);
    setUnderEdit(true);
  };
  const handleUnderEditEnd = () => setUnderEdit(false);

  const handleSaveEdit = () => {
    dispatchProjects({
      type: "rename",
      id: project.id,
      name: underEditText,
    });
    handleUnderEditEnd();
    feedbackSnackbar("Project renamed successfully!", "success");
  };

  return (
    <React.Fragment>
      {/* Projects list item */}
      <ListItem
        key={project.id}
        disablePadding
        secondaryAction={
          <Tooltip title='Project actions'>
            <IconButton onClick={handleProjectActionsMenuClick}>
              <MoreVert />
            </IconButton>
          </Tooltip>
        }
      >
        <ListItemButton
          onClick={handleProjectClick}
          sx={{ paddingY: 3, borderRadius: 2 }}
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
          <Typography
            variant='caption'
            marginLeft={2}
            display={{ xs: "none", lg: "block" }}
          >
            ( {project.renders.length} renders )
          </Typography>
          <Typography
            noWrap
            variant='caption'
            marginLeft={2}
            display={{ xs: "block", lg: "none" }}
          >
            ( {project.renders.length} )
          </Typography>
        </ListItemButton>
      </ListItem>

      {/* Render cards */}
      {/* BUG: In the 'medium' media query, when the renders collapse is open, the layout breaks */}
      <Collapse in={openProject} unmountOnExit>
        <Grid container spacing={2} padding={2}>
          {project.renders.map((render: Render, index) => (
            <Grow
              key={render.id}
              in={openProject}
              style={{ transitionDelay: `${index * 40 + 40}ms` }}
              easing={easing.easeInOut}
            >
              <Grid item xs={6} sm={6} md={4} xl={3} key={render.id}>
                <RenderCard render={render} key={render.id} />
              </Grid>
            </Grow>
          ))}
        </Grid>
      </Collapse>

      {/* Project actions menu */}
      <Menu
        anchorEl={anchorEl}
        open={ProjectActionsMenuOpen}
        onClose={handleProjectActionsMenuClose}
        anchorOrigin={{
          vertical: "bottom",
          horizontal: "right",
        }}
        transformOrigin={{
          vertical: "top",
          horizontal: "right",
        }}
      >
        <MenuItem onClick={handleUnderEditStart}>
          <ListItemIcon>
            <Edit />
          </ListItemIcon>
          Edit name
        </MenuItem>
        <Divider />
        <MenuItem>
          <ListItemIcon>
            <InfoOutlined />
          </ListItemIcon>
          Project details
        </MenuItem>
        <Divider />
        <MenuItem onClick={handleDeleteProjectModalOpen}>
          <ListItemIcon>
            <Delete color='error' />
          </ListItemIcon>
          <Typography sx={{ color: "error.main" }}>Delete project</Typography>
        </MenuItem>
      </Menu>

      {/* Edit project name modal */}
      <Modal
        open={underEdit}
        onClose={handleUnderEditEnd}
        closeAfterTransition
        BackdropComponent={Backdrop}
        BackdropProps={{
          timeout: 500,
        }}
      >
        <Fade in={underEdit}>
          <Box
            sx={{
              position: "absolute" as "absolute",
              top: "50%",
              left: "50%",
              transform: "translate(-50%, -50%)",
              width: { xs: 400, sm: 500, md: 600, lg: 600 },
              bgcolor: "background.paper",
              filter: "drop-shadow(0px 0px 4px rgba(0,0,0,0.5))",
              boxShadow: 24,
              p: 4,
              borderRadius: 2,
            }}
            borderRadius={4}
            display='flex'
            flexDirection='column'
            justifyContent='space-between'
          >
            <Box display='flex' justifyContent='space-between'>
              <Typography
                id='transition-modal-title'
                variant='h6'
                component='h2'
              >
                Edit project name
              </Typography>
              <IconButton onClick={handleUnderEditEnd}>
                <Close />
              </IconButton>
            </Box>
            <Box display='flex' flexDirection='column' marginBottom={3}>
              <TextField
                required
                id='standard-required'
                label='Project name'
                variant='standard'
                margin='normal'
                value={underEditText}
                onChange={(ev) => setUnderEditText(ev.target.value)}
                onKeyPress={(ev) => {
                  if (ev.key === "Enter") {
                    handleSaveEdit();
                  }
                }}
              />
            </Box>
            <Box>
              <Button
                type='submit'
                size='large'
                variant='contained'
                onClick={handleSaveEdit}
              >
                Update
              </Button>
            </Box>
          </Box>
        </Fade>
      </Modal>

      {/* Delete project modal */}
      <Modal
        open={deleteProjectModalOpen}
        onClose={handleDeleteProjectModalClose}
        closeAfterTransition
        BackdropComponent={Backdrop}
        BackdropProps={{
          timeout: 500,
        }}
      >
        <Fade in={deleteProjectModalOpen}>
          <Box
            sx={{
              position: "absolute" as "absolute",
              top: "50%",
              left: "50%",
              transform: "translate(-50%, -50%)",
              width: { xs: 400, sm: 500, md: 600, lg: 600 },
              bgcolor: "background.paper",
              boxShadow: 24,
              p: 4,
            }}
            borderRadius={2}
            display='flex'
            flexDirection='column'
            justifyContent='space-between'
          >
            <Box display='flex' justifyContent='space-between'>
              <Typography
                id='transition-modal-title'
                variant='h6'
                component='h2'
              >
                Delete project
              </Typography>
              <IconButton onClick={handleDeleteProjectModalClose}>
                <Close />
              </IconButton>
            </Box>
            <Box display='flex' flexDirection='column' marginY={3}>
              <Typography variant='body1'>
                Are you sure you want to delete this project?
              </Typography>
              <Typography variant='body1' fontWeight='bold'>
                This action cannot be reversed.
              </Typography>
            </Box>
            <Box>
              <Button
                size='large'
                variant='contained'
                color='error'
                onClick={handleProjectDelete}
              >
                Delete
              </Button>
              <Button
                size='large'
                variant='text'
                color='inherit'
                sx={{ marginLeft: 2 }}
                onClick={handleDeleteProjectModalClose}
              >
                Cancel
              </Button>
            </Box>
          </Box>
        </Fade>
      </Modal>
    </React.Fragment>
  );
}
