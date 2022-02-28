import React from "react";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import IconButton from "@mui/material/IconButton";
import Backdrop from "@mui/material/Backdrop";
import Fade from "@mui/material/Fade";
import Modal from "@mui/material/Modal";

import { Close } from "@mui/icons-material";

import { Project } from "../../../services/projects";

import ProjectDetailsStack from "./ProjectDetailsStack";

interface Props {
  project: Project;
  open: boolean;
  handleClose: () => void;
}

export default function ProjectDetailsModal({
  project,
  open,
  handleClose,
}: Props) {
  const { t } = useTranslation();

  return (
    <Modal
      open={open}
      onClose={handleClose}
      closeAfterTransition
      BackdropComponent={Backdrop}
      BackdropProps={{
        timeout: 500,
      }}
    >
      <Fade in={open}>
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
            <Typography id='transition-modal-title' variant='h6' component='h2'>
              {t("project_details")}
            </Typography>
            <IconButton onClick={handleClose}>
              <Close />
            </IconButton>
          </Box>
          <ProjectDetailsStack project={project} />
        </Box>
      </Fade>
    </Modal>
  );
}
