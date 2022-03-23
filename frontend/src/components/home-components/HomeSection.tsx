import React, { useEffect } from "react";

import AOS from "aos";
import "aos/dist/aos.css";

import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";

import SectionImage from "./SectionImage";
import HomeParagraph from "./HomeParagraph";

interface Props {
  title: string;
  subtitle: string;
  paragraph: string;
  ctaButton?: JSX.Element;
  imageSrc: string;
  imageAlt: string;
  flipped?: boolean;
  hasDecoration?: boolean;
}

export default function HomeSection({
  title,
  subtitle,
  paragraph,
  imageSrc,
  imageAlt,
  ctaButton,
  flipped = false,
  hasDecoration = false,
}: Props) {
  useEffect(() => {
    AOS.init({ duration: 1000 });
  }, []);

  return (
    <>
      <Box
        width='100%'
        display='flex'
        justifyContent='center'
        my={12}
        data-aos='fade-up'
      >
        <Typography
          variant='h4'
          component='h1'
          color='inherit'
          maxWidth='500px'
          alignSelf='center'
          textAlign='center'
          px={2}
        >
          {title}
        </Typography>
      </Box>
      <Box
        width='100%'
        display='flex'
        flexWrap={flipped ? "wrap-reverse" : "wrap"}
        justifyContent='space-evenly'
        alignItems='center'
      >
        {flipped ? (
          <>
            <Box data-aos='zoom-in-right'>
              <SectionImage
                src={imageSrc}
                alt={imageAlt}
                hasDecoration={hasDecoration}
              />
            </Box>
            <Box data-aos='zoom-in-left'>
              <HomeParagraph
                title={subtitle}
                text={paragraph}
                ctaButton={ctaButton}
              />
            </Box>
          </>
        ) : (
          <>
            <Box data-aos='zoom-in-right'>
              <HomeParagraph
                title={subtitle}
                text={paragraph}
                ctaButton={ctaButton}
              />
            </Box>
            <Box data-aos='zoom-in-left'>
              <SectionImage
                src={imageSrc}
                alt={imageAlt}
                hasDecoration={hasDecoration}
              />
            </Box>
          </>
        )}
      </Box>
    </>
  );
}
