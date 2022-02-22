import React from "react";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import Button from "@mui/material/Button";
import IconButton from "@mui/material/IconButton";
import Typography from "@mui/material/Typography";
import Modal from "@mui/material/Modal";
import Backdrop from "@mui/material/Backdrop";
import Fade from "@mui/material/Fade";

import { Close } from "@mui/icons-material";

import { Project, useProjects } from "../../../../services/projects";
import useNotification from "../../../../services/hooks/useNotification";

interface Props {
  project: Project;
  open: boolean;
  handleClose: () => void;
}
export default function DeleteProjectModal({
  project,
  open,
  handleClose,
}: Props) {
  const { t } = useTranslation();

  const { addNotification } = useNotification();

  const { dispatch: dispatchProjects } = useProjects();

  const handleProjectDelete = () => {
    dispatchProjects({
      type: "delete",
      id: project.id,
    });
    handleClose();
    addNotification(t("project_deleted_successfully"), "success");
  };

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
              {t("delete_project")}
            </Typography>
            <IconButton onClick={handleClose}>
              <Close />
            </IconButton>
          </Box>
          <Box display='flex' flexDirection='column' marginY={3}>
            <Typography variant='body1'>
              {t("project_delete_confirm")}
            </Typography>
            <Typography variant='body1' fontWeight='bold'>
              {t("no_reverse")}
            </Typography>
          </Box>
          <Box>
            <Button
              size='large'
              variant='contained'
              color='error'
              onClick={handleProjectDelete}
            >
              {t("delete")}
            </Button>
            <Button
              size='large'
              variant='text'
              color='inherit'
              sx={{ marginLeft: 2 }}
              onClick={handleClose}
            >
              {t("cancel")}
            </Button>
          </Box>
        </Box>
      </Fade>
    </Modal>
  );
}
