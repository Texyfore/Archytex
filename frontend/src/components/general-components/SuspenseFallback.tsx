import React from "react";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import CircularProgress from "@mui/material/CircularProgress";

export default function SuspenseFallback() {
  const { t } = useTranslation();

  return (
    <Box
      display='flex'
      height='100vh'
      justifyContent='center'
      alignItems='center'
      flexDirection='column'
    >
      <CircularProgress color='primary' />
      <Typography marginTop={2}>{t("just_a_moment")}</Typography>
    </Box>
  );
}
