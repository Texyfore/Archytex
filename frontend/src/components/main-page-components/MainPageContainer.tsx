import React from "react";
import { Box } from "@mui/material";

export default function MainPageContainer({
  children,
}: {
  children: JSX.Element;
}) {
  return (
    <Box
      display='flex'
      justifyContent='space-evenly'
      alignItems='center'
      flexDirection={{ xs: "column", md: "row" }}
      marginX={{ xs: 2, md: 10, lg: 20, xl: 40 }}
      marginY='10vh'
    >
      {children}
    </Box>
  );
}
