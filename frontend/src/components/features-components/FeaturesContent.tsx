import React from "react";

import { useTranslation } from "react-i18next";

import HomeSection from "../home-components/HomeSection";

import section1Image from "../../img/illustrations/projects.png";
import section2Image from "../../img/illustrations/editor.png";
import section3Image from "../../img/illustrations/rendered.png";

export default function FeaturesContent() {
  const { t } = useTranslation();

  return (
    <>
      <HomeSection
        title={t("features_section1_title")}
        subtitle={t("features_section1_subtitle")}
        paragraph={t("features_section1_paragraph")}
        imageSrc={section1Image}
        imageAlt='Projects'
        flipped
        hasDecoration
      />
      <HomeSection
        title={t("features_section2_title")}
        subtitle={t("features_section2_subtitle")}
        paragraph={t("features_section2_paragraph")}
        imageSrc={section2Image}
        imageAlt='Editor'
        hasDecoration
      />
      <HomeSection
        title={t("features_section3_title")}
        subtitle={t("features_section3_subtitle")}
        paragraph={t("features_section3_paragraph")}
        imageSrc={section3Image}
        imageAlt='Renders'
        flipped
        hasDecoration
      />
    </>
  );
}
