import {
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
} from "@mui/material";
import React, { useState } from "react";
import { useTheme } from "@mui/material/styles";
import { useProjects } from "../../services/projects";
import CircularProgressWithLabel from "../CircularProgressWithLabel";

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

  //open project details
  const [open, setOpen] = React.useState(false);

  const handleClick = () => {
    setOpen(!open);
  };

  //access theme
  const theme = useTheme();

  //edit project menu
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);
  const editMenuOpen = Boolean(anchorEl);
  const handleEditMenuClick = (event: React.MouseEvent<HTMLButtonElement>) => {
    setAnchorEl(event.currentTarget);
  };
  const handleEditMenuClose = () => {
    setAnchorEl(null);
  };

  //confirm project delete modal
  const [deleteModalOpen, setDeleteModalOpen] = useState(false);
  const handleDeleteModalOpen = () => setDeleteModalOpen(true);
  const handleDeleteModalClose = () => setDeleteModalOpen(false);

  //delete snackbars
  //successful detete snackbar
  const [deletedSnackbarOpen, setDeletedSnackbarOpen] = useState(false);
  const handleDeletedSnackbarClose = () => {
    setDeletedSnackbarOpen(false);
  };
  const handleDeletedSnackbarOpen = () => {
    setDeletedSnackbarOpen(true);
  };

  //project delete handling
  const handleProjectDelete = () => {
    dispatchProjects({
      type: "delete-project",
      id: project.id,
    });
    handleDeleteModalClose();
    handleDeletedSnackbarOpen();
  };

  //title edit handling
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

  return (
    <React.Fragment>
      {/* Projects list item */}
      <ListItem
        disablePadding
        secondaryAction={
          <Tooltip title='Project actions'>
            <IconButton onClick={handleEditMenuClick}>
              <MoreVert />
            </IconButton>
          </Tooltip>
        }
      >
        <ListItemButton
          onClick={handleClick}
          sx={{ paddingY: 3, borderRadius: 2 }}
        >
          <ListItemIcon>
            {open ? <KeyboardArrowDown /> : <KeyboardArrowRight />}
          </ListItemIcon>
          <Typography variant='h6'>{project.name}</Typography>
          <Typography variant='caption' marginLeft={2}>
            ( {project.renders.length} renders )
          </Typography>
        </ListItemButton>
      </ListItem>

      {/* Render cards */}
      <Collapse in={open} unmountOnExit>
        <Grid container spacing={2} padding={2}>
          {project.renders.map((render: RenderModel) => (
            <Grid item xs={6} sm={6} md={4} xl={3}>
              <Card sx={{ maxWidth: 345 }}>
                <CardActionArea
                  disabled={render.status < 100}
                  onClick={() => {
                    console.log("Image clicked");
                  }}
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
          ))}
        </Grid>
      </Collapse>

      {/* Project actions menu */}
      <Menu
        anchorEl={anchorEl}
        open={editMenuOpen}
        onClose={handleEditMenuClose}
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
    </React.Fragment>
  );
}
