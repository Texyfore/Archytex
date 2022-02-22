import React from "react";

import { useTranslation } from "react-i18next";

import { useTheme } from "@mui/material/styles";
import useMediaQuery from "@mui/material/useMediaQuery";
import Typography from "@mui/material/Typography";
import Box from "@mui/material/Box";

import Logo from "../general-components/Logo";
import ParticleBubbles from "./ParticleBubbles";

import headerImage1 from "../../img/illustrations/create_together.svg";
import headerImage2 from "../../img/illustrations/building_1.svg";

export default function HomeHeader() {
  const { t } = useTranslation();

  const theme = useTheme();
  const upMd = useMediaQuery(theme.breakpoints.up("md"));
  const matchesXs = useMediaQuery(theme.breakpoints.only("xs"));

  return (
    <>
      <ParticleBubbles />
      <Box
        position='absolute'
        top={upMd ? "150px" : "300px"}
        right={matchesXs ? "12vw" : "5vw"}
        zIndex={0}
      >
        {upMd ? (
          <img src={headerImage1} alt='header-background' width='500px' />
        ) : (
          <img src={headerImage1} alt='header-background' width='300px' />
        )}
      </Box>
      <Box
        position='absolute'
        top={upMd ? "120px" : "273px"}
        left='0'
        zIndex={0}
        display={{ xs: "none", sm: "inherit" }}
      >
        {upMd ? (
          <img src={headerImage2} alt='header-background' width='500px' />
        ) : (
          <img src={headerImage2} alt='header-background' width='300px' />
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
        <Logo size={100} marginRight={0} />
        <Typography variant='h4' component='h1' color='inherit'>
          {t("archytex").toUpperCase()}
        </Typography>
      </Box>
    </>
  );
}
