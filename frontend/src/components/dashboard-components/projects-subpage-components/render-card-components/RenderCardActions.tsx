import React, { useState } from "react";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import IconButton from "@mui/material/IconButton";
import Tooltip from "@mui/material/Tooltip";
import CardActions from "@mui/material/CardActions";

import { Delete, Download, InfoOutlined, Share } from "@mui/icons-material";

import { Project, Render } from "../../../../services/projects";
import DeleteRenderModal from "./DeleteRenderModal";
import RenderDetailsModal from "./RenderDetailsModal";
import Environment from "../../../../env";

interface Props {
  render: Render;
  project: Project;
}

export default function RenderCardActions({ render, project }: Props) {
  const { t } = useTranslation();
  const downloadTooltipText = t("download");
  const shareTooltipText = t("share");
  const detailsTooltipText = t("details");
  const deleteTooltipText = t("delete");

  const [renderDeleteModalOpen, setRenderDeleteModalOpen] = useState(false);
  const handleDeleteClick = () => {
    setRenderDeleteModalOpen(true);
  };
  const handleDeleteModalClose = () => {
    setRenderDeleteModalOpen(false);
  };

  const [renderDetailsModalOpen, setRenderDetailsModalOpen] = useState(false);
  const handleRenderDetailsModalOpen = () => {
    setRenderDetailsModalOpen(true);
  };
  const handleRenderDetailsModalClose = () => {
    setRenderDetailsModalOpen(false);
  };

  const onDownload = ()=>{
    //TODO: Use this once proxying is set up
    //https://developers.google.com/web/updates/2018/02/chrome-65-deprecations#block_cross-origin_wzxhzdk5a_download
    /*var link = document.createElement("a");
    link.download = `${render.name}.png`;
    link.href = `${Environment.base_url}render/${render.icon}`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);*/
    window.open(`${Environment.base_url}render/${render.icon}`, "_blank");
  }

  return (
    <>
      <CardActions>
        <Box display='flex' justifyContent='space-between' width='100%'>
          <Box>
            <Tooltip title={downloadTooltipText} arrow>
              <span>
                <IconButton
                  size='small'
                  disabled={render.status * 100 < 100}
                  color='success'
                  onClick={onDownload}
                >
                  <Download />
                </IconButton>
              </span>
            </Tooltip>
            <Tooltip title={shareTooltipText} arrow>
              <span>
                <IconButton
                  size='small'
                  disabled={render.status * 100 < 100}
                  color='primary'
                >
                  <Share />
                </IconButton>
              </span>
            </Tooltip>
            <Tooltip title={detailsTooltipText} arrow>
              <span>
                <IconButton size='small' onClick={handleRenderDetailsModalOpen}>
                  <InfoOutlined />
                </IconButton>
              </span>
            </Tooltip>
          </Box>
          <Tooltip title={deleteTooltipText} arrow>
            <span>
              <IconButton
                size='small'
                color='error'
                onClick={handleDeleteClick}
              >
                <Delete />
              </IconButton>
            </span>
          </Tooltip>
        </Box>
      </CardActions>

      <DeleteRenderModal
        project={project}
        render={render}
        open={renderDeleteModalOpen}
        handleClose={handleDeleteModalClose}
      />
      <RenderDetailsModal
        render={render}
        open={renderDetailsModalOpen}
        handleClose={handleRenderDetailsModalClose}
      />
    </>
  );
}
