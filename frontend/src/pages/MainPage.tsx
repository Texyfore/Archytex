import React, { useState } from "react";
import { Button, Typography } from "@mui/material";
import ArcytexAppBar from "../components/ArchytexAppBar";
import MainPageContainer from "../components/main-page-components/MainPageContainer";
import MainPageContentCard from "../components/main-page-components/MainPageContentCard";
import houseImage4 from "../img/4.jpg";
import houseImage6 from "../img/6.jpg";
import houseImage9 from "../img/9.jpg";
import houseImage14 from "../img/14.jpg";
import houseImage11 from "../img/11.jpg";
import planningBoardImage from "../img/planning_board.jpg";
import MainPageParallax from "../components/main-page-components/MainPageParallax";
import ArchytexFooter from "../components/ArchytexFooter";
import AppBarOffset from "../components/AppBarOffset";
import { useTranslation } from "react-i18next";

export default function MainPage() {
  const executeScrollToStart = () =>
    window.scrollTo({ top: 760, behavior: "smooth" });

  const { t } = useTranslation();
  return (
    <React.Fragment>
      <ArcytexAppBar content="general" />
      <AppBarOffset />

      {/* Header */}
      <MainPageParallax
        backgroundOffset='-200px'
        backgroundOpacity={0.7}
        height='98vh'
        imgPath={houseImage9}
      >
        <React.Fragment>
          <Typography
            variant='h1'
            fontWeight={100}
            textAlign='center'
            marginBottom={2}
          >
            {t("archytex")}
          </Typography>
          <Typography variant='subtitle1' textAlign='center' marginBottom={2}>
            {t("motto")}
          </Typography>
          <Button
            variant='outlined'
            size='large'
            sx={{
              marginX: "auto",
            }}
            onClick={executeScrollToStart}
          >
            {t("learn_more")}
          </Button>
        </React.Fragment>
      </MainPageParallax>

      {/* Description */}
      <MainPageContainer>
        <MainPageContentCard
          title={t("what_is_archytex")}
          text={t("p1")}
          imgPath={houseImage4}
        ></MainPageContentCard>
      </MainPageContainer>

      {/* Community Spotlight */}

      {/* In-browser archviz */}
      <MainPageParallax
        imgPath={planningBoardImage}
        height='75vh'
        backgroundOpacity={0.7}
      >
        <React.Fragment>
          <Typography
            variant='h3'
            fontWeight={200}
            textAlign='center'
            marginBottom={2}
          >
            {t("archviz_in_your_browser")}
          </Typography>
          <Button
            variant='text'
            size='large'
            color='inherit'
            sx={{
              marginX: "auto",
            }}
          >
            {t("start_creating_now")}
          </Button>
        </React.Fragment>
      </MainPageParallax>

      <MainPageContainer>
        <MainPageContentCard
          title='Lorem Ipsum'
          text='Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptas obcaecati debitis voluptatem illum sapiente, similique ipsam in ex. Eos at molestiae ratione saepe odio aut maxime accusantium deleniti accusamus architecto?'
          imgPath={houseImage6}
          flipped
        ></MainPageContentCard>
      </MainPageContainer>

      {/* Subscription tiers */}
      <MainPageParallax
        imgPath={houseImage14}
        height='75vh'
        backgroundOpacity={0.7}
      >
        <React.Fragment>
          <Typography
            variant='h3'
            fontWeight={200}
            textAlign='center'
            marginBottom={2}
          >
            {t("blazing_fast_rendering")}
          </Typography>
          <Typography variant='subtitle1' textAlign='center' marginBottom={2}>
            {t("try_it_out")}
          </Typography>
          <Button
            variant='outlined'
            size='large'
            sx={{
              marginX: "auto",
            }}
          >
            {t("subscribe_now")}
          </Button>
        </React.Fragment>
      </MainPageParallax>

      <MainPageContainer>
        <MainPageContentCard
          title='Lorem Ipsum'
          text='Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptas obcaecati debitis voluptatem illum sapiente, similique ipsam in ex. Eos at molestiae ratione saepe odio aut maxime accusantium deleniti accusamus architecto?'
          imgPath={houseImage11}
        ></MainPageContentCard>
      </MainPageContainer>

      {/* Footer */}
      <ArchytexFooter />
    </React.Fragment>
  );
}
