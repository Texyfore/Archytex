import React from "react";

import Box from "@mui/material/Box";

import LibraryCard from "./LibraryCard";

import getProps from "../../../services/libraries/PropItems";

interface Props {
  selected: number | undefined;
  handleSelectionChange: (n: number | undefined) => void;
}
export default function PropLibrary({
  selected,
  handleSelectionChange,
}: Props) {
  const props = getProps();

  return (
    <>
      <Box
        display='flex'
        flexWrap='wrap'
        gap={1}
        alignItems='start'
        justifyContent='space-evenly'
        marginTop={3}
      >
        {props.map((prop, index) => (
          <LibraryCard
            cardType='prop'
            index={index + 1}
            name={prop.name}
            image={prop.thumbnail}
            filterOptions={prop.categories}
            selected={selected}
            handleSelectionChange={handleSelectionChange}
          />
        ))}
      </Box>
    </>
  );
}
