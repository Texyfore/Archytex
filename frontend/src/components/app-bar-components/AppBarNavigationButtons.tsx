import React from "react";

import { Link as L } from "react-router-dom";

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
      <Button variant='text' to='/' component={L}>
        {t("home")}
      </Button>
      <Button variant='text' to='/about' component={L}>
        {t("about")}
      </Button>
      <Button variant='text' to='/dashboard' component={L}>
        {t("dashboard")}
      </Button>
    </Stack>
  );
}
