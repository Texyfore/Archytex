import React from "react";
import { Box, Link, Typography } from "@mui/material";
import { SocialIcon } from "react-social-icons";
import ArchytexIcon from "./ArchytexIcon";
import { useTranslation } from "react-i18next";

export default function ArchytexFooter() {
  const { t } = useTranslation();
  return (
    <Box>
      <Box
        display="flex"
        flexDirection={{ xs: "column", md: "row" }}
        justifyContent="space-evenly"
        alignItems={{ xs: "center", md: "start" }}
        paddingX={{ xs: 1, md: 10 }}
        paddingY={8}
        gap={{ xs: 1, md: 5 }}
      >
        {/* Socials */}
        <Box>
          <Box display="flex" alignItems="center">
            <ArchytexIcon />
            <Typography variant="h6" color="GrayText">
              {t("archytex").toUpperCase()}
            </Typography>
          </Box>
          {/* Socials */}
          <Box display="flex" flexWrap="nowrap" gap={1} marginTop={4}>
            <SocialIcon
              style={{ height: 40, width: 40 }}
              bgColor="transparent"
              fgColor="GrayText"
              network="facebook"
              url="https://facebook.com"
            ></SocialIcon>
            <SocialIcon
              style={{ height: 40, width: 40 }}
              bgColor="transparent"
              fgColor="GrayText"
              network="instagram"
              url="https://instagram.com"
            ></SocialIcon>
            <SocialIcon
              style={{ height: 40, width: 40 }}
              bgColor="transparent"
              fgColor="GrayText"
              network="twitter"
              url="https://twitter.com"
            ></SocialIcon>
            <SocialIcon
              style={{ height: 40, width: 40 }}
              bgColor="transparent"
              fgColor="GrayText"
              network="github"
              url="https://github.com"
            ></SocialIcon>
          </Box>
        </Box>

        {/* Product */}
        <Box
          display="flex"
          flexDirection="column"
          justifyContent="space-evenly"
          gap={1}
        >
          <Typography gutterBottom variant="subtitle2" color="GrayText">
            {t("product").toUpperCase()}
          </Typography>
          <Link variant="body2" href="#">
            {t("features")}
          </Link>
          <Link variant="body2" href="#">
            {t("pricing")}
          </Link>
          <Link variant="body2" href="#">
            {t("faq")}
          </Link>
        </Box>

        {/* Company */}
        <Box
          display="flex"
          flexDirection="column"
          justifyContent="space-evenly"
          gap={1}
        >
          <Typography gutterBottom variant="subtitle2" color="GrayText">
            {t("company").toUpperCase()}
          </Typography>
          <Link variant="body2" href="#">
            {t("about")}
          </Link>
          <Link variant="body2" href="#">
            {t("contact")}
          </Link>
        </Box>
      </Box>

      {/* Copyright */}
      <Box>
        <Typography textAlign="center" color="GrayText" paddingBottom={5}>
          <small>
            {t("copyright")} &copy; {new Date().getFullYear()}, {t("archytex")}
          </small>
        </Typography>
      </Box>
    </Box>
  );
}
