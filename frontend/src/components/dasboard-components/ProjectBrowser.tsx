import * as React from "react";
import Table from "@mui/material/Table";
import TableBody from "@mui/material/TableBody";
import TableCell from "@mui/material/TableCell";
import TableContainer from "@mui/material/TableContainer";
import TableHead from "@mui/material/TableHead";
import TableRow from "@mui/material/TableRow";
import ProjectRow from "./ProjectRow";
import { styled } from "@mui/material/styles";
import {
  Box,
  Backdrop,
  Button,
  Fade,
  IconButton,
  Modal,
  Typography,
  TextField,
} from "@mui/material";
import { Close, LibraryAdd } from "@mui/icons-material";
import SearchBar from "../SearchBar";

const headerHeight = 50;
const projectMenuHeight = 50;
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

function createData(name: string, created: string) {
  return {
    name,
    created,
    renders: [
      {
        renderName:
          name +
          "-project-render-1-and it's very long so it can be abbreviated",
        status: 100, //percentage
        renderTime: "1 h 40 min 23 sec",
      },
      {
        renderName: name + "-project-render-2",
        status: 45, //percentage
        renderTime: "1000h 35 min 21 sec",
      },
    ],
  };
}

const rows = [
  createData("Nice house", "2021.10.25"),
  createData(
    "A house name so long, that is probably doesn't fit on the screen",
    "2021.11.04"
  ),
  createData("Another nice house", "2021.10.26"),
  createData("Another nice house", "2021.10.26"),
  createData("Another nice house", "2021.10.26"),
  createData("Another nice house", "2021.10.26"),
  createData("Another nice house", "2021.10.26"),
];

const ProjectTableContainer = styled(TableContainer)(({ theme }) => ({
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

export default function CollapsibleTable() {
  const [modalOpen, setModalOpen] = React.useState(false);
  const handleModalOpen = () => setModalOpen(true);
  const handleModalClose = () => setModalOpen(false);
  const [expanded, setExpanded] = React.useState<number | false>(false);

  const handleChange =
    (row: number) => (event: React.SyntheticEvent, isExpanded: boolean) => {
      setExpanded(isExpanded ? row : false);
    };

  return (
    <React.Fragment>
      <Box
        height={projectMenuHeight}
        display='flex'
        justifyContent='space-between'
        paddingX={{ xs: 2, sm: 4 }}
      >
        <Box display={{ xs: "none", md: "block" }}>
          <Button
            size='large'
            color='inherit'
            startIcon={<LibraryAdd />}
            onClick={handleModalOpen}
          >
            New project
          </Button>
        </Box>
        <Box display={{ xs: "block", md: "none" }}>
          <IconButton size='large' onClick={handleModalOpen}>
            <LibraryAdd />
          </IconButton>
        </Box>
        <Box>
          <SearchBar />
        </Box>
      </Box>
      <ProjectTableContainer>
        <Table stickyHeader>
          <TableHead>
            <TableRow>
              <TableCell width='10%' sx={{ borderBottom: "2px solid #14151A" }}>
                <Box height='24px' width='24px' padding='10px' />
              </TableCell>
              <TableCell sx={{ borderBottom: "2px solid #14151A" }}>
                NAME
              </TableCell>
              <TableCell
                align='right'
                sx={{ borderBottom: "2px solid #14151A" }}
              >
                CREATED
              </TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            <TableRow>
              <TableCell
                padding='none'
                colSpan={3}
                sx={{ borderBottom: "2px solid #14151A" }}
              >
                {rows.map((row, index) => (
                  <ProjectRow
                    key={index}
                    id={index}
                    row={row}
                    expanded={expanded}
                    handleChange={handleChange}
                  />
                ))}
              </TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </ProjectTableContainer>

      <Modal
        open={modalOpen}
        onClose={handleModalClose}
        closeAfterTransition
        BackdropComponent={Backdrop}
        BackdropProps={{
          timeout: 500,
        }}
      >
        <Fade in={modalOpen}>
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
                Create new project
              </Typography>
              <IconButton onClick={handleModalClose}>
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
              />
            </Box>
            <Box>
              <Button size='large' variant='contained'>
                Create
              </Button>
            </Box>
          </Box>
        </Fade>
      </Modal>
    </React.Fragment>
  );
}
