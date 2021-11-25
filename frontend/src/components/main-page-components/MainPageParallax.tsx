import React from "react";
import { Box } from "@mui/material";
import { useTheme } from "@mui/material/styles";
interface ParallaxProps {
  height: string;
  backgroundOpacity?: Number;
  backgroundOffset?: string | Number;
  imgPath: string;
  children: JSX.Element;
}

export default function MainPageParallax({
  height,
  imgPath,
  backgroundOpacity = 0,
  backgroundOffset = "0px",
  children,
}: ParallaxProps) {
  return (
    <Box
      height={height}
      display='flex'
      flexDirection='column'
      justifyContent='center'
      sx={{
        backgroundImage: `url(${imgPath})`,
        backgroundAttachment: "fixed",
        backgroundPosition: "fixed",
        backgroundPositionY: { xs: 0, lg: `${backgroundOffset}` },
        backgroundRepeat: "no-repeat",
        backgroundSize: "cover",
      }}
    >
      <Box
        paddingY={5}
        width='100%'
        display='flex'
        flexDirection='column'
        justifyContent='center'
        alignItems='center'
        sx={{
          backgroundColor:
            useTheme().palette.mode === "dark"
              ? `rgba(0, 0, 0, ${backgroundOpacity})`
              : `rgba(255, 255, 255, ${backgroundOpacity})`,
        }}
      >
        {children}
      </Box>
    </Box>
  );
}
