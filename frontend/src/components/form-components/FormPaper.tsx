import React from "react";

import Box from "@mui/material/Box";

interface Props {
  children?: JSX.Element | JSX.Element[];
}

export default function FormPaper({ children }: Props) {
  return (
    <Box
      display='flex'
      flexDirection='column'
      justifyContent='center'
      width={{ xs: "100%", md: "unset" }}
      height={{ xs: "100%", md: "unset" }}
      borderRadius={2}
      bgcolor='background.paper'
      sx={{
        filter: { xs: "", md: "drop-shadow(0px 0px 4px rgba(0,0,0,0.25))" },
      }}
    >
      {children}
    </Box>
  );
}
