import React from "react";
import { Link as L } from "react-router-dom";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import Link from "@mui/material/Link";
import Typography from "@mui/material/Typography";
import IconButton from "@mui/material/IconButton";

import {
  FacebookOutlined,
  GitHub,
  Instagram,
  Twitter,
} from "@mui/icons-material";

import Logo from "./Logo";

export default function Footer() {
  const { t } = useTranslation();

  return (
    <Box>
      <Box
        display='flex'
        flexDirection={{ xs: "column", md: "row" }}
        justifyContent='space-evenly'
        alignItems={{ xs: "center", md: "start" }}
        paddingX={{ xs: 1, md: 10 }}
        paddingY={8}
        gap={{ xs: 1, md: 5 }}
      >
        {/* Socials */}
        <Box>
          <Box
            display='flex'
            alignItems='center'
            justifyContent={{ xs: "center", md: "left" }}
          >
            <Logo />
            <Typography variant='h6' color='GrayText'>
              {t("archytex").toUpperCase()}
            </Typography>
          </Box>
          {/* Socials */}
          <Box display='flex' flexWrap='nowrap' gap={1} my={2}>
            <IconButton href='https://facebook.com' color='inherit'>
              <FacebookOutlined />
            </IconButton>
            <IconButton href='https://instagram.com' color='inherit'>
              <Instagram />
            </IconButton>
            <IconButton href='https://twitter.com' color='inherit'>
              <Twitter />
            </IconButton>
            <IconButton href='https://github.com' color='inherit'>
              <GitHub />
            </IconButton>
          </Box>
        </Box>

        {/* Product */}
        <Box
          display='flex'
          flexDirection='column'
          justifyContent='space-evenly'
          gap={1}
          mb={1}
        >
          <Typography gutterBottom variant='subtitle2' color='GrayText'>
            {t("product").toUpperCase()}
          </Typography>
          <Link variant='body2' to='/features' component={L}>
            {t("features")}
          </Link>
        </Box>

        {/* Company */}
        <Box
          display='flex'
          flexDirection='column'
          justifyContent='space-evenly'
          gap={1}
        >
          <Typography gutterBottom variant='subtitle2' color='GrayText'>
            {t("texyfore").toUpperCase()}
          </Typography>
          <Link variant='body2' to='/about' component={L}>
            {t("about")}
          </Link>
        </Box>
      </Box>

      {/* Copyright */}
      <Box>
        <Typography textAlign='center' color='GrayText' paddingBottom={5}>
          <small>
            {t("copyright")} &copy; {new Date().getFullYear()}, {t("archytex")}
          </small>
        </Typography>
      </Box>
    </Box>
  );
}
