import React from "react";

import Box from "@mui/material/Box";

import TextureLibraryCard from "./TextureLibraryCard";

import { Texture } from "../../../services/Library";

interface Props {
  selected: Texture | undefined;
  handleSelectionChange: (texture: Texture | undefined) => void;
  query: string;
  checkedCategories: string[];
  textures: Texture[];
}

export default function TextureLibrary({
  selected,
  handleSelectionChange,
  query,
  checkedCategories,
  textures,
}: Props) {
  const matchesFilter = (texture: Texture) => {
    if (texture.categories.length !== 0) {
      return texture.categories.some((category) =>
        checkedCategories.some(
          (checkedCategory) => checkedCategory === category
        )
      );
    } else return false;
  };
  return (
    <Box
      display='flex'
      flexWrap='wrap'
      gap={1}
      alignItems='start'
      justifyContent='space-evenly'
      mt={3}
    >
      {textures
        .filter(
          (t) =>
            t.name.toLowerCase().includes(query.toLowerCase()) &&
            matchesFilter(t)
        )
        .map((texture, index) => (
          <TextureLibraryCard
            key={index}
            texture={texture}
            isSelected={
              selected === undefined ? false : selected.id === texture.id
            }
            handleSelectionChange={handleSelectionChange}
          />
        ))}
    </Box>
  );
}
