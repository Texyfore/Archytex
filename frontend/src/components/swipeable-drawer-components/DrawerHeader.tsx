import React from "react";

import { useTranslation } from "react-i18next";

import { styled } from "@mui/material/styles";
import Typography from "@mui/material/Typography";

import Logo from "../general-components/Logo";

import { ColorMode, useColorMode } from "../../services/colorMode";

const StyledDrawerHeader = styled("div")(({ theme }) => ({
  display: "flex",
  alignItems: "center",
  justifyContent: "flex-end",
  padding: theme.spacing(0, 1),
  ...theme.mixins.toolbar,
}));

export default function DrawerHeader() {
  const { t } = useTranslation();

  const [colorMode, _] = useColorMode();

  return (
    <StyledDrawerHeader
      sx={{
        width: 300,
        height: 150,
        display: "flex",
        justifyContent: "center",
        backgroundSize: "10px 10px",
        backgroundImage:
          colorMode === ColorMode.Dark
            ? "radial-gradient(#1c517a .75px, #1B1B1A .75px)"
            : "radial-gradient(#1c517a .75px, #f4f4f4 .75px)",
      }}
    >
      <Logo marginRight={0} />
      <Typography variant='h6'>{t("archytex")}</Typography>
    </StyledDrawerHeader>
  );
}
