import { Box, Tooltip, Typography } from "@mui/material";
import React from "react";
import { useTranslation } from "react-i18next";
import ArchytexIcon from "./ArchytexIcon";

const ArchytexLogoWithText = () => {
  const { t } = useTranslation();
  const tooltipText: string =
    t("archytex") + " " + t("version") + " " + "1.0.0";
  return (
    <Tooltip title={tooltipText} placement="bottom-start">
      <Box display={{ xs: "none", md: "flex" }} alignItems="center">
        <ArchytexIcon />
        <Typography
          variant="h6"
          component="h2"
          fontSize="1em"
          sx={{ display: { xs: "none", sm: "block" } }}
        >
          ARCHYTEX
        </Typography>
      </Box>
    </Tooltip>
  );
};

export default ArchytexLogoWithText;
