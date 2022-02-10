import React from "react";

import { useTranslation } from "react-i18next";

import Stack from "@mui/material/Stack";
import Button from "@mui/material/Button";
import Divider from "@mui/material/Divider";

export default function AppBarNavigationButtons() {
  const { t } = useTranslation();
  return (
    <Stack
      direction='row'
      spacing={2}
      divider={<Divider orientation='vertical' flexItem />}
      pl={4}
      display={{ xs: "none", md: "inherit" }}
    >
      <Button variant='text'>{t("home")}</Button>
      <Button variant='text'>{t("about")}</Button>
      <Button variant='text'>{t("dashboard")}</Button>
    </Stack>
  );
}
