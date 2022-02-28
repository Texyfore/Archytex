import React from "react";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";

import HomeSection from "../home-components/HomeSection";

import section1Image from "../../img/illustrations/about_section1.svg";
import section2Image from "../../img/illustrations/about_section2.svg";

export default function AboutContent() {
  const { t } = useTranslation();

  return (
    <Box display='flex' flexWrap='wrap' overflow='hidden'>
      <HomeSection
        title={t("about_section1_title")}
        subtitle={t("about_section1_subtitle")}
        paragraph={t("about_section1_paragraph")}
        imageSrc={section1Image}
        imageAlt='Team'
      />
      <HomeSection
        title={t("about_section2_title")}
        subtitle={t("about_section2_subtitle")}
        paragraph={t("about_section2_paragraph")}
        imageSrc={section2Image}
        imageAlt='Dev Stack'
        flipped
      />
    </Box>
  );
}
