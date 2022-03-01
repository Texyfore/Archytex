import React from "react";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import IconButton from "@mui/material/IconButton";
import Modal from "@mui/material/Modal";
import Tooltip from "@mui/material/Tooltip";
import Grow from "@mui/material/Grow";

import { Close } from "@mui/icons-material";

import { Render } from "../../../services/projects";

interface Props {
  render: undefined | Render;
  handleClose: () => void;
}

export default function EnlargeImageModal({ render, handleClose }: Props) {
  const { t } = useTranslation();
  const closeImageTooltipText = t("close_image");

  return (
    <Modal
      open={render !== undefined}
      onClose={handleClose}
      aria-labelledby='parent-modal-title'
      aria-describedby='parent-modal-description'
      BackdropProps={{
        style: {
          backgroundColor: "rgba(0,0,0, 0.95)",
        },
      }}
      sx={{
        width: "100%",
        height: "100%",
        display: "flex",
        flexDirection: "column",
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <>
        <Grow in={render !== undefined}>
          <Box
            width={{ xs: "98%", md: "60%" }}
            display='flex'
            maxHeight='90%'
            justifyContent='center'
          >
            <img
              width='100%'
              height='undefined'
              style={{ objectFit: "scale-down" }}
              src={render?.icon}
              alt={render?.name}
            />
          </Box>
        </Grow>
        <Box position='absolute' top='5px' right='5px'>
          <Tooltip title={closeImageTooltipText}>
            <IconButton sx={{ color: "#f5f0f6" }} onClick={handleClose}>
              <Close />
            </IconButton>
          </Tooltip>
        </Box>
      </>
    </Modal>
  );
}
