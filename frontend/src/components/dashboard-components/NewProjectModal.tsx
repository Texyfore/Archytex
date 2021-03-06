import React, { useState } from "react";

import { useTranslation } from "react-i18next";

import { AlertColor } from "@mui/material/Alert/Alert";

import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import IconButton from "@mui/material/IconButton";
import Fade from "@mui/material/Fade";
import Modal from "@mui/material/Modal";
import Backdrop from "@mui/material/Backdrop";

import { Close } from "@mui/icons-material";

import FormInput from "../form-components/FormInput";

import { useProjects } from "../../services/projects";
import useNotification from "../../services/hooks/useNotification";

const modalStyle = {
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
};

interface Parameters {
  modalOpen: boolean;
  handleModalOpen: () => void;
  handleModalClose: () => void;
}

export default function NewProjectModal({
  handleModalClose,
  modalOpen,
}: Parameters) {
  const { t } = useTranslation();

  const { addNotification } = useNotification();

  const [name, setName] = useState("");
  const handleNameChange = (e: any) => {
    setErrorMessage("");
    setName(e.target.value);
  };

  const [errorMessage, setErrorMessage] = useState("");
  const handleError = (message: string) => {
    setErrorMessage(message);
  };

  const { dispatch: projectsDispatch } = useProjects();

  const onCreate = (e: any) => {
    e.preventDefault();

    if (name.trim() === "") {
      handleError(t("no_empty_project_name"));
      return;
    }
    if (name.length > 100) {
      handleError(t("long_project_name_error"));
      return;
    }

    projectsDispatch({
      type: "create",
      name: name,
    })
      .then(() => {
        setName("");
        handleClose();
        addNotification(t("project_created_successfully"), "success");
      })
      .catch((error) => {
        handleError(error.message);
      });
  };

  const handleClose = () => {
    handleModalClose();
    setErrorMessage("");
  };

  return (
    <Modal
      open={modalOpen}
      onClose={handleClose}
      closeAfterTransition
      BackdropComponent={Backdrop}
      BackdropProps={{
        timeout: 500,
      }}
    >
      <Fade in={modalOpen}>
        <form onSubmit={onCreate}>
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
                {t("create_new_project")}
              </Typography>
              <IconButton onClick={handleClose}>
                <Close />
              </IconButton>
            </Box>

            <FormInput
              variant={"regular"}
              label={t("project_name")}
              input={name}
              inputChange={handleNameChange}
              error={errorMessage}
            />

            <Box alignSelf='left' marginTop={2}>
              <Button type='submit' size='large' variant='contained'>
                {t("create")}
              </Button>
            </Box>
          </Box>
        </form>
      </Fade>
    </Modal>
  );
}
