import React, { useState } from "react";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import IconButton from "@mui/material/IconButton";
import Fade from "@mui/material/Fade";
import Modal from "@mui/material/Modal";
import Backdrop from "@mui/material/Backdrop";

import { Close } from "@mui/icons-material";

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
  onRender: (width: number, height: number, samples: number) => Promise<void>;
}

export default function RenderSetupModal({
  handleModalClose,
  modalOpen,
  onRender,
}: Parameters) {
  const { t } = useTranslation();

  //Image width
  const [imageWidth, setImageWidth] = useState(1280);
  const handleImageWidthChange = (e: any) => {
    clearErrors();
    setImageWidth(e.target.value);
  };
  const [widthError, setWidthError] = useState("");
  const handleWidthError = (message: string) => {
    setWidthError(message);
  };

  //Image height
  const [imageHeight, setImageHeight] = useState(720);
  const handleImageHeightChange = (e: any) => {
    clearErrors();
    setImageHeight(e.target.value);
  };
  const [heightError, setHeightError] = useState("");
  const handleHeightError = (message: string) => {
    setHeightError(message);
  };

  //Sample count
  const [samples, setSamples] = useState(4);
  const handleSamplesChange = (e: any) => {
    clearErrors();
    setSamples(e.target.value);
  };
  const [samplesError, setSamplesError] = useState("");
  const handleSamplesError = (message: string) => {
    setSamplesError(message);
  };

  const onCreate = (e: any) => {
    e.preventDefault();

    let errored = false;
    if (samples < 1 || samples > 128) {
      handleSamplesError(t("invalid_sample_count") + " | MIN: 1, MAX: 128");
      errored = true;
    }
    if (imageWidth < 100 || imageWidth > 4096 || imageWidth % 4 !== 0) {
      handleWidthError(t("invalid_image_width") + " | MIN: 100px, MAX: 8192px");
      errored = true;
    }
    if (imageHeight < 100 || imageHeight > 4096 || imageHeight % 4 !== 0) {
      handleHeightError(
        t("invalid_image_height") + " | MIN: 100px, MAX: 8192px"
      );
      errored = true;
    }
    if (!errored) {
      handleClose();
      onRender(imageWidth, imageHeight, samples);
    }
    return;
  };

  const clearErrors = () => {
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
                {t("setup_render")}
              </Typography>
              <IconButton onClick={handleClose}>
                <Close />
              </IconButton>
            </Box>
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
              <Button type='submit' size='large' variant='contained'>
                {t("start_render")}
              </Button>
            </Box>
          </Box>
        </form>
      </Fade>
    </Modal>
  );
}
