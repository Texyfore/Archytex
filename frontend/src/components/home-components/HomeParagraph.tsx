import Box from "@mui/material/Box";
import React, { ReactElement } from "react";
import Typography from "@mui/material/Typography";

interface Props {
  title: string;
  text: string;
  ctaButton?: ReactElement;
}

export default function HomeParagraph({ title, text, ctaButton }: Props) {
  return (
    <Box
      display='flex'
      flexDirection='column'
      alignItems='left'
      mb={{ xs: 10, md: 0 }}
    >
      <Typography
        variant='h5'
        maxWidth='400px'
        component='h2'
        color='primary'
        px={2}
        mb={4}
      >
        {title}
      </Typography>
      <Typography
        variant='body1'
        maxWidth='400px'
        textAlign='justify'
        px={2}
        mb={2}
      >
        {text}
      </Typography>
      <Box alignSelf='left' ml={2}>
        {ctaButton}
      </Box>
    </Box>
  );
}
