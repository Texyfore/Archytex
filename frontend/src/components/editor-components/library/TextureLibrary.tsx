import { Box } from "@mui/material";
import React from "react";
import LibraryCard from "./LibraryCard";

export default function TextureLibrary() {
  return (
    <Box
      display='flex'
      flexWrap='wrap'
      gap={2}
      alignItems='start'
      justifyContent='space-evenly'
    >
      {[...new Array(21)].map(() => (
        <LibraryCard />
      ))}
    </Box>
  );
}
