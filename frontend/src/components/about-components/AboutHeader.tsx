import React from "react";

import { useTranslation } from "react-i18next";

import { useTheme } from "@mui/material/styles";
import useMediaQuery from "@mui/material/useMediaQuery";
import Typography from "@mui/material/Typography";
import Box from "@mui/material/Box";

import AboutParticleBubbles from "./AboutParticleBubbles";

export default function AboutHeader() {
  const { t } = useTranslation();

  const theme = useTheme();
  const upMd = useMediaQuery(theme.breakpoints.up("md"));
  const matchesXs = useMediaQuery(theme.breakpoints.only("xs"));

  return (
    <>
      <AboutParticleBubbles />

      <Box
        position='absolute'
        top='500px'
        width='100%'
        zIndex={1}
        bgcolor='#444053'
        height='1.5px'
      />
      <Box
        position='absolute'
        top={0}
        width='100%'
        height={matchesXs ? "400px" : "500px"}
        display='flex'
        flexDirection='column'
        justifyContent='center'
        alignItems='center'
      >
        <Typography variant='h4' component='h1' color='inherit'>
          {t("about").toUpperCase()}
        </Typography>
      </Box>
    </>
  );
}
