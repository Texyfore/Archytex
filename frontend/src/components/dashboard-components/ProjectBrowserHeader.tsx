import React, { useState } from "react";

import { useTranslation } from "react-i18next";

import useTheme from "@mui/material/styles/useTheme";

import Button from "@mui/material/Button";
import Box from "@mui/material/Box";
import Tooltip from "@mui/material/Tooltip";
import IconButton from "@mui/material/IconButton";

import { LibraryAdd } from "@mui/icons-material";

import SearchBar from "../general-components/SearchBar";
import NewProjectModal from "./NewProjectModal";

interface Props {
  query: string;
  handleQueryChange: (query: string) => void;
}
export default function ProjectBrowserHeader({
  query,
  handleQueryChange,
}: Props) {
  const { t } = useTranslation();
  const tooltipText = t("create_new_project");

  const [newProjectModalOpen, setNewProjectModalOpen] = useState(false);
  const handleNewProjectModalOpen = () => setNewProjectModalOpen(true);
  const handleNewProjectModalClose = () => setNewProjectModalOpen(false);

  return (
    <>
      <Box
        height='60px'
        display='flex'
        justifyContent='space-between'
        alignItems='center'
        paddingX={{ xs: 2, sm: 4 }}
        borderBottom={`1px solid ${
          useTheme().palette.mode === "dark" ? "#2E2E2E" : "#BABABA"
        }`}
      >
        <Box display={{ xs: "none", md: "block" }}>
          <Button
            size='large'
            color='inherit'
            startIcon={<LibraryAdd />}
            onClick={handleNewProjectModalOpen}
          >
            {t("new_project")}
          </Button>
        </Box>

        <Box display={{ xs: "block", md: "none" }}>
          <Tooltip title={tooltipText}>
            <IconButton size='large' onClick={handleNewProjectModalOpen}>
              <LibraryAdd />
            </IconButton>
          </Tooltip>
        </Box>

        <Box>
          <SearchBar query={query} handleQueryChange={handleQueryChange} />
        </Box>
      </Box>

      <NewProjectModal
        modalOpen={newProjectModalOpen}
        handleModalOpen={handleNewProjectModalOpen}
        handleModalClose={handleNewProjectModalClose}
      />
    </>
  );
}
