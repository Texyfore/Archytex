import React from "react";

import Box from "@mui/material/Box";

interface Props {
  src: string;
  alt: string;
}

export default function SectionImage({ src, alt }: Props) {
  return (
    <Box width={{ xs: "300px", md: "300px", xl: "400px" }}>
      <img src={src} alt={alt} width='100%' style={{ objectFit: "contain" }} />
    </Box>
  );
}
