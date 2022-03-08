import React from "react";

import Box from "@mui/material/Box";

import LibraryCard from "./LibraryCard";
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
    if (texture.public !== null) {
      return texture.public.categories.some((category) =>
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
          <LibraryCard
            key={index}
            cardType='texture'
            item={texture}
            isSelected={
              selected === undefined ? false : selected.id === texture.id
            }
            handleSelectionChange={handleSelectionChange}
          />
        ))}
    </Box>
  );
}
