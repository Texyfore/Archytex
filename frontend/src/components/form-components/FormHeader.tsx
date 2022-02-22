import React from "react";

import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";

interface Props {
  title: string;
}

export default function FormHeader({ title }: Props) {
  return (
    <Box
      width='304px'
      marginX='auto'
      display='flex'
      alignItems='center'
      justifyContent='center'
      marginTop={3}
    >
      <Box
        flexGrow={1}
        height={1.01}
        sx={{ backgroundColor: "primary.main" }}
      />
      <Typography variant='h6' fontWeight={600} fontSize='1em' paddingX={2}>
        {title}
      </Typography>
      <Box
        flexGrow={1}
        height={1.01}
        sx={{ backgroundColor: "primary.main" }}
      />
    </Box>
  );
}
