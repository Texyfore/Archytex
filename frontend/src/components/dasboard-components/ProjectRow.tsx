import React, { SyntheticEvent, useEffect, useState } from "react";
import {
  Accordion,
  AccordionDetails,
  AccordionSummary,
  Alert,
  Backdrop,
  Box,
  Button,
  Divider,
  Fade,
  IconButton,
  LinearProgress,
  LinearProgressProps,
  ListItemIcon,
  Menu,
  MenuItem,
  Modal,
  Snackbar,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableRow,
  TextField,
  Typography,
} from "@mui/material";
import { styled } from "@mui/material/styles";
import {
  Close,
  Delete,
  Edit,
  KeyboardArrowDown,
  KeyboardArrowRight,
} from "@mui/icons-material";

const modalStyle = {
  position: "absolute" as "absolute",
  top: "50%",
  left: "50%",
  transform: "translate(-50%, -50%)",
  width: { xs: 400, sm: 500, md: 600, lg: 600 },
  bgcolor: "background.paper",
  border: "1px solid #14151A",
  boxShadow: 24,
  p: 4,
};

const ProjectTableCell = styled(TableCell)(({ theme }) => ({
  padding: "none",
  margin: "none",
  borderBottom: "none",
}));

function LinearProgressWithLabel(
  props: LinearProgressProps & { value: number }
) {
  return (
    <Box sx={{ display: "flex", alignItems: "center" }}>
      <Box sx={{ width: "100%", mr: 1 }}>
        <LinearProgress variant='determinate' {...props} />
      </Box>
      <Box sx={{ minWidth: 35 }}>
        <Typography variant='body2' color='text.secondary'>{`${Math.round(
          props.value
        )}%`}</Typography>
      </Box>
    </Box>
  );
}

function createData(name: string, created: string) {
  return {
    name,
    created,
    renders: [
      {
        renderName: name + "- it's very long so it can be abbreviated",
        status: 100, //percentage
        renderTime: "1000 h 40 min 23 sec",
      },
      {
        renderName: name + "-project-render-2",
        status: 45, //percentage
        renderTime: "35 min 21 sec",
      },
    ],
  };
}

export default function ProjectRow(props: {
  id: number;
  row: ReturnType<typeof createData>;
  expanded: number | boolean;
  handleChange: (
    row: number
  ) =>
    | ((event: SyntheticEvent<Element, Event>, expanded: boolean) => void)
    | undefined;
}) {
  //props
  const { row, id, expanded, handleChange } = props;

  //progress bar
  const [progress, setProgress] = useState(10);
  useEffect(() => {
    const timer = setInterval(() => {
      setProgress((prevProgress) =>
        prevProgress >= 100 ? 10 : prevProgress + 10
      );
    }, 800);
    return () => {
      clearInterval(timer);
    };
  }, []);

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
    handleDeleteModalClose();
    handleDeletedSnackbarOpen();
  };

  //title edit handling
  const [underEdit, setUnderEdit] = useState(false);
  const handleUnderEditStart = () => setUnderEdit(true);
  const handleUnderEditEnd = () => setUnderEdit(false);

  const handleSaveEdit = () => {
    handleUnderEditEnd();
  };

  return (
    <Accordion
      disableGutters
      elevation={0}
      expanded={expanded === id}
      onChange={handleChange(id)}
      sx={
        expanded === id
          ? {
              backgroundColor: "#1F1F1F",
              borderRadius: 4,
            }
          : {
              position: "static",
              borderRadius: 4,
              ".MuiAccordionSummary-root:hover": {
                backgroundColor: "#1F1F1F",
              },
            }
      }
    >
      <AccordionSummary sx={{ paddingX: 0 }}>
        <Table>
          <TableBody>
            <TableRow>
              <ProjectTableCell align='left' width='10%'>
                <IconButton
                  aria-label='expand row'
                  size='small'
                  onClick={() => {
                    handleChange(id);
                  }}
                >
                  {expanded === id ? (
                    <KeyboardArrowDown />
                  ) : (
                    <KeyboardArrowRight />
                  )}
                </IconButton>
              </ProjectTableCell>
              <ProjectTableCell align='left' width='auto'>
                <React.Fragment>
                  <Box display={underEdit ? "none" : "block"}>
                    <Typography variant='body1' textAlign='start'>
                      {row.name}
                    </Typography>
                  </Box>
                  <Box display={underEdit ? "flex" : "none"} gap={1}>
                    <TextField
                      autoFocus
                      defaultValue={row.name}
                      id='standard-required'
                      label='Project name'
                      variant='standard'
                      margin='none'
                    />
                    <Button onClick={handleSaveEdit}>Save</Button>
                    <Button onClick={handleUnderEditEnd} color='inherit'>
                      Cancel
                    </Button>
                  </Box>
                </React.Fragment>
              </ProjectTableCell>
              <ProjectTableCell align='right'>{row.created}</ProjectTableCell>
            </TableRow>
          </TableBody>
        </Table>
      </AccordionSummary>
      <AccordionDetails sx={{ paddingLeft: { md: 10, lg: 12 } }}>
        <Box
          display='flex'
          justifyContent='end'
          paddingY={1}
          borderBottom='1px solid #F5F0F6'
        >
          <Button
            startIcon={<Edit />}
            color='inherit'
            onClick={handleEditMenuClick}
          >
            Edit project
          </Button>
          <Menu
            id='basic-menu'
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
            <MenuItem onClick={handleDeleteModalOpen}>
              <ListItemIcon>
                <Delete color='error' />
              </ListItemIcon>
              <Typography sx={{ color: "#fb4d3d" }}>Delete project</Typography>
            </MenuItem>
          </Menu>
        </Box>

        <Modal
          open={deleteModalOpen}
          onClose={handleDeleteModalClose}
          closeAfterTransition
          BackdropComponent={Backdrop}
          BackdropProps={{
            timeout: 500,
          }}
        >
          <Fade in={deleteModalOpen}>
            <Box
              sx={modalStyle}
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
                  Delete project
                </Typography>
                <IconButton onClick={handleDeleteModalClose}>
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
                  onClick={handleDeleteModalClose}
                >
                  Cancel
                </Button>
              </Box>
            </Box>
          </Fade>
        </Modal>

        <Snackbar
          open={deletedSnackbarOpen}
          autoHideDuration={6000}
          onClose={handleDeletedSnackbarClose}
        >
          <Alert
            onClose={handleDeletedSnackbarClose}
            severity='success'
            sx={{ width: "100%" }}
          >
            Project deleted successfully!
          </Alert>
        </Snackbar>

        {/* TODO: RendersTable component */}
        <Table>
          <TableHead>
            <TableRow sx={{ fontStyle: "italic" }}>
              <ProjectTableCell align='left'>Renders</ProjectTableCell>
              <ProjectTableCell align='left'>Status</ProjectTableCell>
              <ProjectTableCell align='right'>Render time</ProjectTableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {row.renders.map((render) => (
              <TableRow>
                <ProjectTableCell align='left'>
                  <Box display={{ xs: "none", sm: "block" }}>
                    <Typography
                      noWrap
                      width={{ xs: 200, sm: 200, md: 200, lg: 200, xl: 350 }}
                    >
                      {render.renderName}
                    </Typography>
                  </Box>
                  <Box display={{ xs: "block", sm: "none" }}>
                    <Typography
                      width={{ xs: 200, sm: 200, md: 200, lg: 200, xl: 350 }}
                    >
                      {render.renderName}
                    </Typography>
                  </Box>
                </ProjectTableCell>
                <ProjectTableCell align='left' width='100%'>
                  <Box display={{ xs: "none", md: "block" }}>
                    <LinearProgressWithLabel value={progress} />
                  </Box>
                  <Box display={{ xs: "block", md: "none" }}>
                    <Typography noWrap>{progress}%</Typography>
                  </Box>
                </ProjectTableCell>
                <ProjectTableCell align='right'>
                  <Typography width={{ xs: 50, sm: 150 }}>
                    {render.renderTime}
                  </Typography>
                </ProjectTableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </AccordionDetails>
    </Accordion>
  );
}
