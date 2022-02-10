import React from "react";
import Typography from "@mui/material/Typography";
import Box from "@mui/material/Box";
import Logo from "../general-components/Logo";
import headerBackground from "../../img/headerBackground.svg";

export default function HomeHeader() {
  return (
    <>
      <Box
        display='flex'
        flexDirection='column'
        justifyContent='center'
        alignItems='center'
        width='100%'
        height='75vh'
      >
        <img src={headerBackground} alt='header-background' width='100%' />
      </Box>
      <Box
        position='absolute'
        top={0}
        width='100%'
        height={{ xs: "50vh", md: "75vh" }}
        display='flex'
        flexDirection='column'
        justifyContent='center'
        alignItems='center'
      >
        <Logo size={100} />
        <Typography variant='h4' component='h1' color='initial'>
          ARCHYTEX
        </Typography>
      </Box>
    </>
  );
}
