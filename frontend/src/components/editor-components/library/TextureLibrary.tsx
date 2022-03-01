import React from "react";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";

import LibraryCard from "./LibraryCard";

import getTextures from "../../../services/libraries/TextureItems";

interface Props {
  selected: number | undefined;
  handleSelectionChange: (id: number | undefined) => void;
}

export default function TextureLibrary({
  selected,
  handleSelectionChange,
}: Props) {
  const { t } = useTranslation();

  const textures = getTextures();

  return (
    <>
      <Box
        display='flex'
        flexWrap='wrap'
        gap={1}
        alignItems='start'
        justifyContent='space-evenly'
        mt={3}
      >
        {textures.map((texture, index) => (
          <LibraryCard
            cardType='texture'
            index={index + 1}
            name={texture.name}
            image={texture.thumbnail}
            filterOptions={texture.categories}
            selected={selected}
            handleSelectionChange={handleSelectionChange}
          />
        ))}
      </Box>
    </>
  );
}
