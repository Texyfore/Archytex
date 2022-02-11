import React, { ReactElement } from "react";

import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";

import SectionImage from "./SectionImage";
import HomeParagraph from "./HomeParagraph";

interface Props {
  title: string;
  subtitle: string;
  paragraph: string;
  ctaButton?: ReactElement;
  imageSrc: string;
  imageAlt: string;
  flipped?: boolean;
}

export default function HomeSection({
  title,
  subtitle,
  paragraph,
  imageSrc,
  imageAlt,
  ctaButton,
  flipped = false,
}: Props) {
  return (
    <>
      <Box width='100%' display='flex' justifyContent='center' my={12}>
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
            <SectionImage src={imageSrc} alt={imageAlt} />
            <HomeParagraph
              title={subtitle}
              text={paragraph}
              ctaButton={ctaButton}
            />
          </>
        ) : (
          <>
            <HomeParagraph
              title={subtitle}
              text={paragraph}
              ctaButton={ctaButton}
            />
            <SectionImage src={imageSrc} alt={imageAlt} />
          </>
        )}
      </Box>
    </>
  );
}
