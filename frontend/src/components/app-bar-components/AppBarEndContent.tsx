import React from "react";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import Button from "@mui/material/Button";
import LanguageSelectDropdown from "../general-components/LanguageSelectDropdown";
import DarkModeSwitch from "../general-components/DarkModeSwitch";

export default function AppBarEndContent() {
  const { t } = useTranslation();

  return (
    <Box
      alignSelf='end'
      display='flex'
      justifyContent='end'
      gap={1}
      alignItems='center'
    >
      <Box display={{ xs: "none", md: "inherit" }}>
        <LanguageSelectDropdown />
        <DarkModeSwitch />
      </Box>
      <Button variant='outlined'>{t("login")}</Button>
    </Box>
  );
}
