import React, { useState } from "react";

import { useHistory } from "react-router-dom";

import { useTranslation } from "react-i18next";

import Typography from "@mui/material/Typography";
import Menu from "@mui/material/Menu";
import MenuItem from "@mui/material/MenuItem";
import Divider from "@mui/material/Divider";
import ListItemIcon from "@mui/material/ListItemIcon";

import { Delete, Edit, InfoOutlined, Send } from "@mui/icons-material";

import EditProjectModal from "./EditProjectModal";
import DeleteProjectModal from "./DeleteProjectModal";

import { Project } from "../../../../services/projects";
import ProjectDetailsModal from "./ProjectDetailsModal";

interface Props {
  project: Project;
  anchorEl: null | HTMLElement;
  open: boolean;
  handleClose: () => void;
}

export default function ProjectActionsMenu({
  project,
  open,
  anchorEl,
  handleClose,
}: Props) {
  const { t } = useTranslation();

  const history = useHistory();

  //Confirm project delete modal
  const [deleteProjectModalOpen, setDeleteProjectModalOpen] = useState(false);
  const handleDeleteProjectModalOpen = () => {
    setDeleteProjectModalOpen(true);
    handleClose();
  };
  const handleDeleteProjectModalClose = () => {
    setDeleteProjectModalOpen(false);
  };

  //Project details modal
  const [projectDetailsModalOpen, setProjectDetailsModalOpen] = useState(false);
  const handleProjectDetailsModalOpen = () => {
    setProjectDetailsModalOpen(true);
    handleClose();
  };
  const handleProjectDetailsModalClose = () => {
    setProjectDetailsModalOpen(false);
  };

  //Title edit handling
  const [underEdit, setUnderEdit] = useState(false);
  const handleUnderEditStart = () => {
    handleClose();
    setUnderEdit(true);
  };
  const handleUnderEditEnd = () => setUnderEdit(false);

  return (
    <>
      <Menu
        anchorEl={anchorEl}
        open={open}
        onClose={handleClose}
        anchorOrigin={{
          vertical: "bottom",
          horizontal: "right",
        }}
        transformOrigin={{
          vertical: "top",
          horizontal: "right",
        }}
      >
        <MenuItem onClick={() => history.push(`/editor/${project.id}`)}>
          <ListItemIcon>
            <Send />
          </ListItemIcon>
          {t("open_in_editor")}
        </MenuItem>
        <Divider />
        <MenuItem onClick={handleUnderEditStart}>
          <ListItemIcon>
            <Edit />
          </ListItemIcon>
          {t("edit_name")}
        </MenuItem>
        <Divider />
        <MenuItem onClick={handleProjectDetailsModalOpen}>
          <ListItemIcon>
            <InfoOutlined />
          </ListItemIcon>
          {t("project_details")}
        </MenuItem>
        <Divider />
        <MenuItem onClick={handleDeleteProjectModalOpen}>
          <ListItemIcon>
            <Delete color='error' />
          </ListItemIcon>
          <Typography sx={{ color: "error.main" }}>
            {t("delete_project")}
          </Typography>
        </MenuItem>
      </Menu>

      <EditProjectModal
        project={project}
        open={underEdit}
        handleClose={handleUnderEditEnd}
      />

      <DeleteProjectModal
        project={project}
        open={deleteProjectModalOpen}
        handleClose={handleDeleteProjectModalClose}
      />

      <ProjectDetailsModal
        project={project}
        open={projectDetailsModalOpen}
        handleClose={handleProjectDetailsModalClose}
      />
    </>
  );
}
