import React from "react";
import { Box, Typography } from "@mui/material";
import LibraryCard from "./LibraryCard";
import brownPlanks from "../../../img/texture_thumbnails/brown_planks_07.jpg";
import concreteFloor from "../../../img/texture_thumbnails/concrete_floor_worn_001.jpg";
import concreteWall from "../../../img/texture_thumbnails/concrete_wall_001.jpg";
import largeFloorTiles from "../../../img/texture_thumbnails/large_floor_tiles_02.jpg";
import redBrick from "../../../img/texture_thumbnails/red_brick_03.jpg";
import roof from "../../../img/texture_thumbnails/roof_09.jpg";
import weatheredBrownPlanks from "../../../img/texture_thumbnails/weathered_brown_planks.jpg";
interface TextureLibraryProps {
  selected: number | undefined;
  handleSelectionChange: (n: number | undefined) => void;
}

enum FilterOptions {
  brick = "Brick",
  wood = "Wood",
  concrete = "Concrete",
  rock = "Rock",
  dirty = "Dirty",
  clean = "Clean",
}
interface Texture {
  name: string;
  filterOptions: FilterOptions[];
  image: string;
}

export default function TextureLibrary({
  selected,
  handleSelectionChange,
}: TextureLibraryProps) {
  const textures: Texture[] = [
    {
      name: "Concrete floor",
      filterOptions: [FilterOptions.concrete, FilterOptions.dirty],
      image: concreteFloor,
    },
    {
      name: "Large floor tiles",
      filterOptions: [FilterOptions.rock, FilterOptions.dirty],
      image: largeFloorTiles,
    },
    {
      name: "Red brick",
      filterOptions: [FilterOptions.brick, FilterOptions.dirty],
      image: redBrick,
    },
    {
      name: "Brown planks",
      filterOptions: [FilterOptions.wood, FilterOptions.clean],
      image: brownPlanks,
    },
    {
      name: "Weathered brown planks",
      filterOptions: [FilterOptions.wood, FilterOptions.dirty],
      image: weatheredBrownPlanks,
    },
    {
      name: "Concrete wall",
      filterOptions: [FilterOptions.concrete, FilterOptions.clean],
      image: concreteWall,
    },
    {
      name: "Roof",
      filterOptions: [FilterOptions.dirty],
      image: roof,
    },
  ];
  const recentTextures: Texture[] = [
    {
      name: "Brown planks",
      filterOptions: [FilterOptions.wood, FilterOptions.clean],
      image: brownPlanks,
    },
    {
      name: "Concrete wall",
      filterOptions: [FilterOptions.concrete, FilterOptions.clean],
      image: concreteWall,
    },
  ];

  return (
    <>
      <Typography paddingTop={2} gutterBottom color='GrayText'>
        Recent
      </Typography>
      <Box
        display='flex'
        flexWrap='wrap'
        gap={1}
        alignItems='start'
        justifyContent='space-evenly'
        paddingBottom={3}
        marginBottom={3}
        borderBottom='1px solid GrayText'
      >
        {recentTextures.map((texture, index) => (
          <LibraryCard
            index={index}
            name={texture.name}
            image={texture.image}
            filterOptions={texture.filterOptions}
            selected={selected}
            handleSelectionChange={handleSelectionChange}
          />
        ))}
      </Box>
      <Box
        display='flex'
        flexWrap='wrap'
        gap={1}
        alignItems='start'
        justifyContent='space-evenly'
      >
        {textures.map((texture, index) => (
          <LibraryCard
            index={index + 6}
            name={texture.name}
            image={texture.image}
            filterOptions={texture.filterOptions}
            selected={selected}
            handleSelectionChange={handleSelectionChange}
          />
        ))}
      </Box>
    </>
  );
}
