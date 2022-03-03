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

import useNotification from "../../services/hooks/useNotification";
import FormInput from "../form-components/FormInput";

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

export default function RednerSetupModal({
  handleModalClose,
  modalOpen,
}: Parameters) {
  const { t } = useTranslation();

  const { addNotification } = useNotification();

  //Render name
  const [name, setName] = useState("projectname_render_1");
  const handleNameChange = (e: any) => {
    clearErrors();
    setName(e.target.value);
  };
  const [nameError, setNameError] = useState("");
  const handleNameError = (message: string) => {
    setNameError(message);
  };

  //Image width
  const [imageWidth, setImageWidth] = useState(1920);
  const handleImageWidthChange = (e: any) => {
    clearErrors();
    setImageWidth(e.target.value);
  };
  const [widthError, setWidthError] = useState("");
  const handleWidthError = (message: string) => {
    setWidthError(message);
  };

  //Image height
  const [imageHeight, setImageHeight] = useState(1080);
  const handleImageHeightChange = (e: any) => {
    clearErrors();
    setImageHeight(e.target.value);
  };
  const [heightError, setHeightError] = useState("");
  const handleHeightError = (message: string) => {
    setHeightError(message);
  };

  //Sample count
  const [samples, setSamples] = useState(32);
  const handleSamplesChange = (e: any) => {
    clearErrors();
    setSamples(e.target.value);
  };
  const [samplesError, setSamplesError] = useState("");
  const handleSamplesError = (message: string) => {
    setSamplesError(message);
  };

  const onCreate = () => {
    let errored = false;
    if (name.trim() === "") {
      handleNameError(t("no_empty_render_name"));
      errored = true;
    }
    if (samples < 1) {
      handleSamplesError(t("invalid_sample_count"));
      errored = true;
    }
    if (imageWidth < 100 || imageWidth > 4096) {
      handleWidthError("invalid_image_width");
      errored = true;
    }
    if (imageHeight < 100 || imageHeight > 4096) {
      handleHeightError(t("invalid_image_height"));
      errored = true;
    }
    if (!errored) {
      //TODO: Send settings and start render
      handleClose();
      addNotification(t("rendering_started"), "info");
    }
    return;
  };

  const clearErrors = () => {
    setNameError("");
    setHeightError("");
    setWidthError("");
    setSamplesError("");
  };

  const handleClose = () => {
    handleModalClose();
    clearErrors();
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
        <Box
          sx={modalStyle}
          borderRadius={4}
          display='flex'
          flexDirection='column'
          justifyContent='space-between'
        >
          <Box display='flex' justifyContent='space-between'>
            <Typography id='transition-modal-title' variant='h6' component='h2'>
              {t("setup_render")}
            </Typography>
            <IconButton onClick={handleClose}>
              <Close />
            </IconButton>
          </Box>

          <FormInput
            variant='regular'
            label={t("render_name")}
            input={name}
            inputChange={handleNameChange}
            error={nameError}
          />
          <Box display='flex' justifyContent='space-evenly' gap={2}>
            <Box>
              <FormInput
                variant='number'
                label={t("image_width")}
                input={imageWidth}
                inputChange={handleImageWidthChange}
                error={widthError}
              />
            </Box>
            <Box>
              <FormInput
                variant='number'
                label={t("image_height")}
                input={imageHeight}
                inputChange={handleImageHeightChange}
                error={heightError}
              />
            </Box>
            <Box>
              <FormInput
                variant='number'
                label={t("sample_count")}
                input={samples}
                inputChange={handleSamplesChange}
                error={samplesError}
              />
            </Box>
          </Box>

          <Box alignSelf='left' marginTop={2}>
            <Button
              type='submit'
              size='large'
              variant='contained'
              onClick={onCreate}
            >
              {t("start_render")}
            </Button>
          </Box>
        </Box>
      </Fade>
    </Modal>
  );
}
