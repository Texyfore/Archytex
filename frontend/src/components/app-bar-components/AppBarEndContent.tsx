import React from "react";

import { Link as L } from "react-router-dom";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import Button from "@mui/material/Button";
import LanguageSelectDropdown from "../general-components/LanguageSelectDropdown";
import DarkModeSwitch from "../general-components/DarkModeSwitch";

import { useApi } from "../../services/user/api";
import UserIconButton from "./UserIconButton";

export default function AppBarEndContent() {
  const { t } = useTranslation();

  const api = useApi();

  return (
    <Box
      alignSelf='center'
      display='flex'
      justifyContent='end'
      gap={1}
      alignItems='center'
    >
      <Box display={{ xs: "none", md: "inherit" }}>
        <LanguageSelectDropdown />
        <DarkModeSwitch />
      </Box>
      {api?.state !== "logged-in" ? (
        <Button variant='outlined' to='/login' component={L}>
          {t("login")}
        </Button>
      ) : (
        <UserIconButton />
      )}
    </Box>
  );
}
