import React from "react";
import { Link as L } from "react-router-dom";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import Link from "@mui/material/Link";
import Typography from "@mui/material/Typography";
import IconButton from "@mui/material/IconButton";
import Tooltip from "@mui/material/Tooltip";

import { Article, GitHub } from "@mui/icons-material";

import Logo from "./Logo";

export default function Footer() {
  const { t } = useTranslation();
  const documentationTooltip = t("documentation");

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
          <Box
            display='flex'
            flexWrap='nowrap'
            gap={1}
            my={2}
            justifyContent={{ xs: "center", md: "left" }}
          >
            <Tooltip title='GitHub'>
              <IconButton
                href='https://github.com/Texyfore/Archytex'
                color='inherit'
              >
                <GitHub />
              </IconButton>
            </Tooltip>
            <Tooltip title={documentationTooltip}>
              <IconButton
                href='https://drive.google.com/file/d/1P_kkBg1wiy4Kdl5p-TVFptlt4jo2VUjT/view'
                color='inherit'
              >
                <Article />
              </IconButton>
            </Tooltip>
          </Box>
        </Box>

        {/* Product */}
        <Box
          display='flex'
          flexDirection='column'
          justifyContent='space-evenly'
          gap={1}
          mb={1}
          textAlign={{ xs: "center", md: "left" }}
        >
          <Box
            height='32px'
            display='flex'
            alignItems='center'
            justifyContent='center'
          >
            <Typography variant='subtitle2' color='GrayText'>
              {t("product").toUpperCase()}
            </Typography>
          </Box>
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
          textAlign={{ xs: "center", md: "left" }}
        >
          <Box
            height='32px'
            display='flex'
            alignItems='center'
            justifyContent='center'
          >
            <Typography variant='subtitle2' color='GrayText'>
              {t("texyfore").toUpperCase()}
            </Typography>
          </Box>
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
