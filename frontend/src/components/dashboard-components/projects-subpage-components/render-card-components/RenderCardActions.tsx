import React from "react";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import IconButton from "@mui/material/IconButton";
import Tooltip from "@mui/material/Tooltip";
import CardActions from "@mui/material/CardActions";

import { Delete, Download, InfoOutlined, Share } from "@mui/icons-material";

import { Render } from "../../../../services/projects";

interface Props {
  render: Render;
}

export default function RenderCardActions({ render }: Props) {
  const { t } = useTranslation();
  const downloadTooltipText = t("download");
  const shareTooltipText = t("share");
  const detailsTooltipText = t("details");
  const deleteTooltipText = t("delete");

  return (
    <CardActions>
      <Box display='flex' justifyContent='space-between' width='100%'>
        <Box>
          <Tooltip title={downloadTooltipText} arrow>
            <span>
              <IconButton
                size='small'
                disabled={render.status * 100 < 100}
                color='success'
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
              <IconButton size='small'>
                <InfoOutlined />
              </IconButton>
            </span>
          </Tooltip>
        </Box>
        <Tooltip title={deleteTooltipText} arrow>
          <span>
            <IconButton size='small' color='error'>
              <Delete />
            </IconButton>
          </span>
        </Tooltip>
      </Box>
    </CardActions>
  );
}
