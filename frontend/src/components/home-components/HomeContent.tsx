import React from "react";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import Button from "@mui/material/Button";

import { NavigateNext } from "@mui/icons-material";

import HomeSection from "./HomeSection";

import section1Image from "../../img/illustrations/section1.svg";
import section2Image from "../../img/illustrations/section2.svg";
import section3Image from "../../img/illustrations/section3.svg";
import section4Image from "../../img/illustrations/section4.svg";

export default function HomeContent() {
  const { t } = useTranslation();

  return (
    <Box display='flex' flexWrap='wrap'>
      <HomeSection
        title={t("section1_title")}
        subtitle={t("section1_subtitle")}
        paragraph={t("section1_paragraph")}
        ctaButton={
          <Button
            onClick={() => console.log("click")}
            endIcon={<NavigateNext />}
          >
            {t("section1_cta")}
          </Button>
        }
        imageSrc={section1Image}
        imageAlt='3D artist'
      />
      <HomeSection
        title={t("section2_title")}
        subtitle={t("section2_subtitle")}
        paragraph={t("section2_paragraph")}
        ctaButton={
          <Button
            onClick={() => console.log("click")}
            endIcon={<NavigateNext />}
          >
            {t("section2_cta")}
          </Button>
        }
        imageSrc={section2Image}
        imageAlt='3D artist'
        flipped
      />
      <HomeSection
        title={t("section3_title")}
        subtitle={t("section3_subtitle")}
        paragraph={t("section3_paragraph")}
        ctaButton={
          <Button
            onClick={() => console.log("click")}
            endIcon={<NavigateNext />}
          >
            {t("section3_cta")}
          </Button>
        }
        imageSrc={section3Image}
        imageAlt='3D artist'
      />
      <HomeSection
        title={t("section4_title")}
        subtitle={t("section4_subtitle")}
        paragraph={t("section4_paragraph")}
        ctaButton={
          <Button
            onClick={() => console.log("click")}
            endIcon={<NavigateNext />}
          >
            {t("section4_cta")}
          </Button>
        }
        imageSrc={section4Image}
        imageAlt='3D artist'
        flipped
      />
    </Box>
  );
}
