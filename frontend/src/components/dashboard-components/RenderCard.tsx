import React, { useState } from "react";
import {
  Box,
  Card,
  CardActionArea,
  CardActions,
  CardContent,
  CardMedia,
  Grow,
  IconButton,
  Modal,
  Tooltip,
  Typography,
} from "@mui/material";
import { Close, Download, InfoOutlined, Share } from "@mui/icons-material";
import CircularProgressWithLabel from "../CircularProgressWithLabel";
import { Render } from "../../services/projects";
import { useTranslation } from "react-i18next";

interface RenderCardProps {
  render: Render;
}

export default function RenderCard({ render }: RenderCardProps) {
  const { t } = useTranslation();

  //Enlarge render image modal
  const [openEnlargeRenderModal, setOpenEnlargeRenderModal] = useState<
    undefined | Render
  >(undefined);
  const handleOpenEnlargeRenderModal = (render: Render) => {
    setOpenEnlargeRenderModal(render);
  };
  const handleCloseEnlargeRenderModal = () =>
    setOpenEnlargeRenderModal(undefined);

  const downloadTooltipText = t("download");
  const shareTooltipText = t("share");
  const detailsTooltipText = t("details");
  const closeImageTooltipText = t("close_image");

  return (
    <React.Fragment>
      <Card sx={{ maxWidth: 345 }} key={render.id}>
        <CardActionArea
          disabled={render.status < 100}
          onClick={() => handleOpenEnlargeRenderModal(render)}
        >
          <CardMedia
            component='img'
            sx={{
              height: { xs: "150px", sm: "200px", md: "250px" },
            }}
            image={render.icon}
            alt='green iguana'
          />
          {/* Image overlay for progress information */}
          <Box
            position='relative'
            width='100%'
            height={0}
            display={render.status < 100 ? "block" : "none"}
          >
            <Box
              position='absolute'
              top={{ xs: "-150px", sm: "-200px", md: "-250px" }}
              height={{ xs: "150px", sm: "200px", md: "250px" }}
              width='100%'
              display='flex'
              justifyContent='center'
              alignItems='center'
              sx={{
                backgroundColor: "rgba(0, 0, 0, 0.7)",
              }}
            >
              <Box>
                <CircularProgressWithLabel
                  size={80}
                  thickness={1}
                  value={render.status * 100}
                />
              </Box>
            </Box>
          </Box>
          <Tooltip title={render.name} placement='top'>
            <CardContent sx={{ maxHeight: "100px" }}>
              <Typography variant='h6' component='div' noWrap>
                {render.name}
              </Typography>
            </CardContent>
          </Tooltip>
        </CardActionArea>
        <CardActions>
          <Tooltip title={downloadTooltipText} arrow>
            <span>
              <IconButton size='small' disabled={render.status * 100 < 100}>
                <Download />
              </IconButton>
            </span>
          </Tooltip>
          <Tooltip title={shareTooltipText} arrow>
            <span>
              <IconButton size='small' disabled={render.status * 100 < 100}>
                <Share />
              </IconButton>
            </span>
          </Tooltip>
          <Tooltip title={detailsTooltipText} arrow>
            <span>
              <IconButton size='small'>
                <InfoOutlined />
              </IconButton>
            </span>
          </Tooltip>
        </CardActions>
      </Card>

      {/* Enlarge render image modal */}
      <Modal
        open={openEnlargeRenderModal !== undefined}
        onClose={handleCloseEnlargeRenderModal}
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
        <React.Fragment>
          <Grow in={openEnlargeRenderModal !== undefined}>
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
                src={openEnlargeRenderModal?.icon}
                alt={openEnlargeRenderModal?.name}
              />
            </Box>
          </Grow>
          <Box position='absolute' top='5px' right='5px'>
            <Tooltip title={closeImageTooltipText}>
              <IconButton
                sx={{ color: "#f5f0f6" }}
                onClick={handleCloseEnlargeRenderModal}
              >
                <Close />
              </IconButton>
            </Tooltip>
          </Box>
        </React.Fragment>
      </Modal>
    </React.Fragment>
  );
}
