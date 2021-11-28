import {
  Close,
  Delete,
  Download,
  Edit,
  InfoOutlined,
  KeyboardArrowDown,
  KeyboardArrowRight,
  MoreVert,
  Share,
} from "@mui/icons-material";
import {
  Card,
  CardActionArea,
  CardActions,
  CardContent,
  CardMedia,
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
} from "@mui/material";
import React, { useState } from "react";
import { useTheme } from "@mui/material/styles";
import { useProjects } from "../../services/projects";
import CircularProgressWithLabel from "../CircularProgressWithLabel";
import { render } from "@testing-library/react";

interface ProjectModel {
  id: string;
  created: string;
  name: string;
  renders: RenderModel[];
}
interface RenderModel {
  renderName: string;
  img: string;
  status: number;
}

interface RenderRowProps {
  project: ProjectModel;
}

export default function RenderRow({ project }: RenderRowProps) {
  const { dispatch: dispatchProjects } = useProjects();

  //Open project collapse
  const [openProject, setOpenProject] = React.useState(false);

  const handleProjectClick = () => {
    setOpenProject(!openProject);
  };

  //access theme
  const theme = useTheme();

  //Edit project menu
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);
  const editProjectMenuOpen = Boolean(anchorEl);
  const handleEditProjectMenuClick = (
    event: React.MouseEvent<HTMLButtonElement>
  ) => {
    setAnchorEl(event.currentTarget);
  };
  const handleEditProjectMenuClose = () => {
    setAnchorEl(null);
  };

  //confirm project delete modal
  const [deleteModalOpen, setDeleteModalOpen] = useState(false);
  const handleDeleteModalOpen = () => setDeleteModalOpen(true);
  const handleDeleteModalClose = () => setDeleteModalOpen(false);

  //Snackbars
  //Successful detete snackbar
  const [deletedSnackbarOpen, setDeletedSnackbarOpen] = useState(false);
  const handleDeletedSnackbarClose = () => {
    setDeletedSnackbarOpen(false);
  };
  const handleDeletedSnackbarOpen = () => {
    setDeletedSnackbarOpen(true);
  };

  //Project delete handling
  const handleProjectDelete = () => {
    dispatchProjects({
      type: "delete-project",
      id: project.id,
    });
    handleDeleteModalClose();
    handleDeletedSnackbarOpen();
  };

  //Title edit handling
  const [underEdit, setUnderEdit] = useState(false);
  const [underEditText, setUnderEditText] = useState("");
  const handleUnderEditStart = () => {
    setUnderEditText(project.name);
    setUnderEdit(true);
  };
  const handleUnderEditEnd = () => setUnderEdit(false);

  const handleSaveEdit = () => {
    dispatchProjects({
      type: "rename-project",
      id: project.id,
      name: underEditText,
    });
    handleUnderEditEnd();
  };

  //Enlarge render image modal
  const [openEnlargeRenderModal, setOpenEnlargeRenderModal] = useState<
    undefined | RenderModel
  >(undefined);
  const handleOpenEnlargeRenderModal = (render: RenderModel) => {
    setOpenEnlargeRenderModal(render);
  };
  const handleCloseEnlargeRenderModal = () =>
    setOpenEnlargeRenderModal(undefined);
  return (
    <React.Fragment>
      {/* Projects list item */}
      <ListItem
        disablePadding
        secondaryAction={
          <Tooltip title='Project actions'>
            <IconButton onClick={handleEditProjectMenuClick}>
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
          <Typography variant='h6'>{project.name}</Typography>
          <Typography variant='caption' marginLeft={2}>
            ( {project.renders.length} renders )
          </Typography>
        </ListItemButton>
      </ListItem>

      {/* Render cards */}
      <Collapse in={openProject} unmountOnExit>
        <Grid container spacing={2} padding={2}>
          {project.renders.map((render: RenderModel) => (
            <React.Fragment>
              <Grid item xs={6} sm={6} md={4} xl={3}>
                <Card sx={{ maxWidth: 345 }}>
                  <CardActionArea
                    disabled={render.status < 100}
                    onClick={() => handleOpenEnlargeRenderModal(render)}
                  >
                    <CardMedia
                      component='img'
                      sx={{
                        height: { xs: "150px", sm: "200px", md: "250px" },
                      }}
                      image={render.img}
                      alt='green iguana'
                    />
                    {/* Image overlay for progress information */}
                    <Box
                      position='relative'
                      width='100%'
                      height={0}
                      display={render.status < 100 ? "block" : "none"}
                    >
                      <Box
                        position='absolute'
                        top={{ xs: "-150px", sm: "-200px", md: "-250px" }}
                        height={{ xs: "150px", sm: "200px", md: "250px" }}
                        width='100%'
                        display='flex'
                        justifyContent='center'
                        alignItems='center'
                        sx={{
                          backgroundColor: "rgba(0, 0, 0, 0.7)",
                        }}
                      >
                        <Box>
                          <CircularProgressWithLabel
                            size={80}
                            thickness={1}
                            value={render.status}
                          />
                        </Box>
                      </Box>
                    </Box>
                    <Tooltip title={render.renderName} placement='top'>
                      <CardContent sx={{ maxHeight: "100px" }}>
                        <Typography variant='h6' component='div' noWrap>
                          {render.renderName}
                        </Typography>
                      </CardContent>
                    </Tooltip>
                  </CardActionArea>
                  <CardActions>
                    <Tooltip title='Download'>
                      <IconButton size='small' disabled={render.status < 100}>
                        <Download />
                      </IconButton>
                    </Tooltip>
                    <Tooltip title='Share'>
                      <IconButton size='small' disabled={render.status < 100}>
                        <Share />
                      </IconButton>
                    </Tooltip>
                    <Tooltip title='Details'>
                      <IconButton size='small'>
                        <InfoOutlined />
                      </IconButton>
                    </Tooltip>
                  </CardActions>
                </Card>
              </Grid>
            </React.Fragment>
          ))}
        </Grid>
      </Collapse>

      {/* Project actions menu */}
      <Menu
        anchorEl={anchorEl}
        open={editProjectMenuOpen}
        onClose={handleEditProjectMenuClose}
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
        <MenuItem onClick={handleDeleteModalOpen}>
          <ListItemIcon>
            <Delete color='error' />
          </ListItemIcon>
          <Typography sx={{ color: "error.main" }}>Delete project</Typography>
        </MenuItem>
      </Menu>

      {/* Enlarge render image modal */}
      <Modal
        open={openEnlargeRenderModal !== undefined}
        onClose={handleCloseEnlargeRenderModal}
        aria-labelledby='parent-modal-title'
        aria-describedby='parent-modal-description'
        BackdropProps={{
          style: {
            backgroundColor: "rgba(0,0,0, 0.95)",
          },
        }}
        sx={{
          width: "100%",
          height: "100%",
          display: "flex",
          flexDirection: "column",
          justifyContent: "center",
          alignItems: "center",
        }}
      >
        <React.Fragment>
          <Box
            width={{ xs: "98%", md: "60%" }}
            display='flex'
            maxHeight='90%'
            justifyContent='center'
          >
            <img
              width='100%'
              height='undefined'
              style={{ objectFit: "scale-down" }}
              src={openEnlargeRenderModal?.img}
              alt={openEnlargeRenderModal?.renderName}
            />
          </Box>
          <Box position='absolute' top='5px' right='5px'>
            <Tooltip title='Close image'>
              <IconButton
                sx={{ color: "#f5f0f6" }}
                onClick={handleCloseEnlargeRenderModal}
              >
                <Close />
              </IconButton>
            </Tooltip>
          </Box>
        </React.Fragment>
      </Modal>
    </React.Fragment>
  );
}
