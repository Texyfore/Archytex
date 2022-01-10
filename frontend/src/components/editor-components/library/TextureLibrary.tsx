import React, { useState } from "react";
import { Box, Typography } from "@mui/material";
import LibraryCard from "./LibraryCard";

export default function TextureLibrary() {
  const [selected, setSelected] = useState<number | undefined>(undefined);
  const handleSelection = (n: number | undefined) => {
    setSelected(n);
  };
  return (
    <>
      <Typography paddingTop={2} gutterBottom color='GrayText'>
        Recent
      </Typography>
      <Box
        display='flex'
        flexWrap='wrap'
        gap={2}
        alignItems='start'
        justifyContent='space-evenly'
        paddingBottom={3}
        marginBottom={3}
        borderBottom='1px solid GrayText'
      >
        {[...new Array(6)].map((_, index) => (
          <LibraryCard
            index={index}
            selected={selected}
            handleSelection={handleSelection}
          />
        ))}
      </Box>
      <Box
        display='flex'
        flexWrap='wrap'
        gap={2}
        alignItems='start'
        justifyContent='space-evenly'
      >
        {[...new Array(21)].map((_, index) => (
          <LibraryCard
            index={index + 6}
            selected={selected}
            handleSelection={handleSelection}
          />
        ))}
      </Box>
    </>
  );
}
