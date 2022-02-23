import React from "react";

import Box from "@mui/material/Box";
import Tooltip from "@mui/material/Tooltip";
import Typography from "@mui/material/Typography";

import { useTranslation } from "react-i18next";

import Logo from "./Logo";

export default function LogoWithText() {
  const { t } = useTranslation();
  const tooltipText: string =
    t("archytex") + " " + t("version") + " " + t("version_number");

  return (
    <Tooltip title={tooltipText} placement='bottom-start'>
      <Box display={{ xs: "none", md: "flex" }} alignItems='center'>
        <Logo />
        <Typography
          variant='h6'
          component='h1'
          fontSize='1em'
          sx={{ display: { xs: "none", sm: "initial" } }}
        >
          {t("archytex").toUpperCase()}
        </Typography>
      </Box>
    </Tooltip>
  );
}
