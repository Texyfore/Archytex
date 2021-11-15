import React from "react";
import { Box, Typography } from "@mui/material";

interface CardProps {
  title: string;
  text: string;
  imgPath: string;
  flipped?: Boolean;
}

export default function MainPageContentCard({
  title,
  text,
  imgPath,
  flipped,
}: CardProps) {
  if (flipped) {
    return (
      <React.Fragment>
        <Box
          sx={{
            backgroundColor: "transparent",
            backgroundSize: "10px 10px",
            backgroundImage:
              "radial-gradient(#1c517a .75px, transparent .75px)",
          }}
        >
          <Box
            position='relative'
            top='-30px'
            left='30px'
            width={{ xs: "300px", md: "400px" }}
          >
            <img width='100%' src={imgPath} alt='A nice house' />
          </Box>
        </Box>
        <Box
          padding={5}
          marginLeft={{ md: 5 }}
          width={{ sm: "80%", lg: "40%" }}
        >
          <Typography
            variant='h3'
            fontWeight={200}
            marginBottom={6}
            textAlign='left'
          >
            {title}
          </Typography>
          <Typography variant='body1' fontWeight={200} textAlign='justify'>
            {text}
          </Typography>
        </Box>
      </React.Fragment>
    );
  }
  return (
    <React.Fragment>
      <Box padding={5} width={{ sm: "80%", lg: "40%" }}>
        <Typography
          variant='h3'
          fontWeight={200}
          marginBottom={6}
          textAlign='left'
        >
          {title}
        </Typography>
        <Typography
          variant='body1'
          fontWeight={200}
          textAlign='justify'
          marginBottom={{ xs: 5, md: 0 }}
        >
          {text}
        </Typography>
      </Box>
      <Box
        sx={{
          backgroundColor: "transparent",
          backgroundSize: "10px 10px",
          backgroundImage: "radial-gradient(#1c517a .75px, transparent .75px)",
        }}
      >
        <Box
          position='relative'
          top='-30px'
          left='30px'
          width={{ xs: "300px", md: "400px" }}
        >
          <img width='100%' src={imgPath} alt='A nice house' />
        </Box>
      </Box>
    </React.Fragment>
  );
}
