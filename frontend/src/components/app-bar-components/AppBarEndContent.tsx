import React from "react";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import Button from "@mui/material/Button";

export default function AppBarEndContent() {
  const { t } = useTranslation();

  return (
    <Box alignSelf='end' pb={0.3}>
      <Button variant='outlined'>{t("login")}</Button>
    </Box>
  );
}
