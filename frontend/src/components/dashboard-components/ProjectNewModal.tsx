import { Close } from "@mui/icons-material";
import {
  AlertColor,
  Backdrop,
  Box,
  Button,
  Fade,
  IconButton,
  Modal,
  TextField,
  Typography,
} from "@mui/material";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { useProjects } from "../../services/projects";

const modalStyle = {
  position: "absolute" as "absolute",
  top: "50%",
  left: "50%",
  transform: "translate(-50%, -50%)",
  width: { xs: 400, sm: 500, md: 600, lg: 600 },
  bgcolor: "background.paper",
  //   border: "1px solid #14151A",
  filter: "drop-shadow(0px 0px 4px rgba(0,0,0,0.5))",
  boxShadow: 24,
  p: 4,
  borderRadius: 2,
};

interface Parameters {
  modalOpen: boolean;
  handleModalOpen: () => void;
  handleModalClose: () => void;
  feedbackSnackbar: (text: string, severity: AlertColor) => void;
}

export default function ProjectNewModal({
  handleModalClose,
  handleModalOpen,
  modalOpen,
  feedbackSnackbar,
  ...params
}: Parameters) {
  //i18n
  const { t } = useTranslation();

  const [name, setName] = useState("");
  const { dispatch: projectsDispatch } = useProjects();
  const onCreate = () => {
    if (name.trim() !== "") {
      //TODO: Handle errors
      projectsDispatch({
        type: "create",
        name: name,
      });
      setName("");
      handleModalClose();
      feedbackSnackbar(t("project_created_successfully"), "success");
    }
  };
  return (
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
            <Typography id='transition-modal-title' variant='h6' component='h2'>
              {t("create_new_project")}
            </Typography>
            <IconButton onClick={handleModalClose}>
              <Close />
            </IconButton>
          </Box>
          <Box display='flex' flexDirection='column' marginBottom={3}>
            <TextField
              required
              id='standard-required'
              label={t("project_name")}
              variant='standard'
              margin='normal'
              value={name}
              onChange={(ev) => setName(ev.target.value)}
              onKeyPress={(ev) => {
                if (ev.key === "Enter") {
                  onCreate();
                }
              }}
            />
          </Box>
          <Box>
            <Button
              type='submit'
              size='large'
              variant='contained'
              onClick={onCreate}
            >
              {t("create")}
            </Button>
          </Box>
        </Box>
      </Fade>
    </Modal>
  );
}
