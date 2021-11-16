import React, { useState } from "react";
import ProjectRow from "./ProjectRow";
import SearchBar from "../SearchBar";
import { styled } from "@mui/material/styles";
import {
  Box,
  Button,
  IconButton,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
} from "@mui/material";
import { LibraryAdd } from "@mui/icons-material";
import { useProjects } from "../../services/projects";
import ProjectNewModal from "./ProjectNewModal";

const headerHeight = 50;
const projectMenuHeight = 50;
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


export default function ProjectBrowser() {
  //new project modal
  const [modalOpen, setModalOpen] = useState(false);
  const handleModalOpen = () => setModalOpen(true);
  const handleModalClose = () => setModalOpen(false);
  const { state: projects } = useProjects();

  //expanded project
  const [expanded, setExpanded] = useState<string | false>(false);
  const handleChange =
    (row: string) => (event: React.SyntheticEvent, isExpanded: boolean) => {
      setExpanded(isExpanded ? row : false);
    };

  //project manager context
  //TODO
  //const projects = useProjects();

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
                {projects.projects.map((project, index) => (
                  <ProjectRow
                    key={project.id}
                    row={project}
                    expanded={expanded}
                    handleChange={handleChange}
                  />
                ))}
              </TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </ProjectTableContainer>

      <ProjectNewModal
        handleModalClose={handleModalClose}
        handleModalOpen={handleModalOpen}
        modalOpen={modalOpen}
      ></ProjectNewModal>
    </React.Fragment>
  );
}
