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

export default function MainPage() {
  const [open, setOpen] = useState(false);
  function handleOpenChange(value: boolean): void {
    setOpen(value);
  }
  const executeScrollToStart = () =>
    window.scrollTo({ top: 760, behavior: "smooth" });

  return (
    <React.Fragment>
      <ArcytexAppBar handleOpenChange={handleOpenChange} open={open} />
      <AppBarOffset />

      {/* Header */}
      <MainPageParallax
        backgroundOffset='-200px'
        backgroundOpacity={0.5}
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
            Archytex
          </Typography>
          <Typography variant='subtitle1' textAlign='center' marginBottom={2}>
            Take your architectural visualisations to the next level
          </Typography>
          <Button
            variant='outlined'
            size='large'
            sx={{
              marginX: "auto",
            }}
            onClick={executeScrollToStart}
          >
            Learn more
          </Button>
        </React.Fragment>
      </MainPageParallax>

      {/* Description */}
      <MainPageContainer>
        <MainPageContentCard
          title='What is Archytex?'
          text='Archytex is a lightweight 3D architecture design tool, combined with fast and
          powerful ray-traced rendering. All while staying in your web browser.'
          imgPath={houseImage4}
        ></MainPageContentCard>
      </MainPageContainer>

      {/* Community Spotlight */}

      {/* In-browser archviz */}
      <MainPageParallax
        imgPath={planningBoardImage}
        height='50vh'
        backgroundOpacity={0.5}
      >
        <React.Fragment>
          <Typography
            variant='h3'
            fontWeight={100}
            textAlign='center'
            marginBottom={2}
          >
            Archviz in your browser
          </Typography>
          <Button
            variant='text'
            size='large'
            color='inherit'
            sx={{
              marginX: "auto",
            }}
          >
            Start creating now
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
        height='50vh'
        backgroundOpacity={0.5}
      >
        <React.Fragment>
          <Typography
            variant='h3'
            fontWeight={100}
            textAlign='center'
            marginBottom={2}
          >
            Blazing fast rendering
          </Typography>
          <Typography variant='subtitle1' textAlign='center' marginBottom={2}>
            Try our server side ray-traced rendering solution, now with a 1
            month free trial
          </Typography>
          <Button
            variant='outlined'
            size='large'
            sx={{
              marginX: "auto",
            }}
          >
            Subscribe now
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
