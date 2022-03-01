import React, { useState } from "react";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";

import LibraryCard from "./LibraryCard";

import Texture from "../../../services/types/Texture";
import getTextures from "../../../services/libraries/TextureItems";

interface Props {
  selected: Texture;
  handleSelectionChange: (texture: Texture) => void;
}

export default function TextureLibrary({
  selected,
  handleSelectionChange,
}: Props) {
  const { t } = useTranslation();

  const textures = getTextures();

  const [selectedIndex, setSelectedIndex] = useState<number | undefined>(
    selected.id
  );
  const handleIndexChange = (index: number | undefined) => {
    setSelectedIndex(index);
  };

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
            index={index}
            name={texture.name}
            image={texture.thumbnail}
            filterOptions={texture.categories}
            selected={selectedIndex}
            handleSelectionChange={handleIndexChange}
          />
        ))}
      </Box>
    </>
  );
}
