import React from "react";

import { useTranslation } from "react-i18next";

import { useTheme } from "@mui/material/styles";
import useMediaQuery from "@mui/material/useMediaQuery";
import Typography from "@mui/material/Typography";
import Box from "@mui/material/Box";

import ParticleBubbles from "../home-components/ParticleBubbles";

import headerImage1 from "../../img/illustrations/features.svg";

export default function FeaturesHeader() {
  const { t } = useTranslation();

  const theme = useTheme();
  const upMd = useMediaQuery(theme.breakpoints.up("md"));
  const matchesXs = useMediaQuery(theme.breakpoints.only("xs"));

  return (
    <>
      <ParticleBubbles />
      <Box
        position='absolute'
        top={upMd ? "173px" : "300px"}
        right={matchesXs ? "12vw" : "5vw"}
        zIndex={0}
      >
        {upMd ? (
          <img src={headerImage1} alt='header-background' width='350px' />
        ) : (
          <img src={headerImage1} alt='header-background' width='200px' />
        )}
      </Box>
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
          {t("features").toUpperCase()}
        </Typography>
      </Box>
    </>
  );
}
