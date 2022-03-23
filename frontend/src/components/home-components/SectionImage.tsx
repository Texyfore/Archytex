import React from "react";

import Box from "@mui/material/Box";

interface Props {
  src: string;
  alt: string;
  hasDecoration?: boolean;
}

export default function SectionImage({
  src,
  alt,
  hasDecoration = false,
}: Props) {
  return hasDecoration ? (
    <Box
      sx={{
        backgroundColor: "transparent",
        backgroundSize: "8px 8px",
        backgroundImage: "radial-gradient(#1c517a .75px, transparent .75px)",
      }}
    >
      <Box
        position='relative'
        top='-30px'
        left='30px'
        width={{ xs: "300px", md: "400px" }}
      >
        <img width='100%' src={src} alt='A nice house' />
      </Box>
    </Box>
  ) : (
    <Box width={{ xs: "300px", md: "300px", xl: "400px" }}>
      <img src={src} alt={alt} width='100%' style={{ objectFit: "contain" }} />
    </Box>
  );
}
