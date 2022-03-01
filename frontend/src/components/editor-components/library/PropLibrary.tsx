import React from "react";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";

import alarmClock from "../../../img/prop_thumbnails/alarm_clock_01.png";
import ceramicVase from "../../../img/prop_thumbnails/ceramic_vase_01.png";
import roundCoffeeTable from "../../../img/prop_thumbnails/coffee_table_round_01.png";
import horseStatue from "../../../img/prop_thumbnails/horse_statue_01.png";
import modernCoffeeTable from "../../../img/prop_thumbnails/modern_coffee_table_01.png";
import ottoman from "../../../img/prop_thumbnails/ottoman_01.png";
import paintedWoodenChair from "../../../img/prop_thumbnails/painted_wooden_chair_01.png";
import pottedPlant from "../../../img/prop_thumbnails/potted_plant_04.png";
import standingPictureFrame from "../../../img/prop_thumbnails/standing_picture_frame_01.png";
import steelFramShelves from "../../../img/prop_thumbnails/steel_frame_shelves_01.png";

import LibraryCard from "./LibraryCard";

import Prop from "../../../services/types/Prop";

interface Props {
  selected: number | undefined;
  handleSelectionChange: (n: number | undefined) => void;
}
export default function PropLibrary({
  selected,
  handleSelectionChange,
}: Props) {
  const { t } = useTranslation();

  const props: Prop[] = [
    {
      id: 1,
      name: "Alarm clock",
      categories: ["Decoration"],
      thumbnail: alarmClock,
    },
    {
      id: 2,
      name: "Ceramic vase",
      categories: ["Decoration"],
      thumbnail: ceramicVase,
    },
    {
      id: 3,
      name: "Round coffee table",
      categories: ["Furniture", "Table"],
      thumbnail: roundCoffeeTable,
    },
    {
      id: 4,
      name: "Horse statue",
      categories: ["Decoration"],
      thumbnail: horseStatue,
    },
    {
      id: 5,
      name: "Modern coffee table",
      categories: ["Furniture", "Table"],
      thumbnail: modernCoffeeTable,
    },
    {
      id: 6,
      name: "Ottoman",
      categories: ["Furniture", "Chair"],
      thumbnail: ottoman,
    },
    {
      id: 7,
      name: "Painted wooden chair",
      categories: ["Furniture", "Chair"],
      thumbnail: paintedWoodenChair,
    },
    {
      id: 8,
      name: "Potted plant",
      categories: ["Decoration"],
      thumbnail: pottedPlant,
    },
    {
      id: 9,
      name: "Standing picture frame",
      categories: ["Decoration"],
      thumbnail: standingPictureFrame,
    },
    {
      id: 10,
      name: "Steel frame shelves",
      categories: ["Furniture"],
      thumbnail: steelFramShelves,
    },
  ];
  const recentProps: Prop[] = [
    // {
    //   id: 1,
    //   name: "Alarm clock",
    //   categories: [PropFilterOptions.decoration],
    //   thumbnail: alarmClock,
    // },
  ];

  return (
    <>
      <Box display={recentProps.length === 0 ? "none" : "initial"}>
        <Typography paddingTop={2} gutterBottom color='GrayText'>
          {t("recent")}
        </Typography>
        <Box
          display='flex'
          flexWrap='wrap'
          gap={1}
          alignItems='start'
          justifyContent='space-evenly'
          pb={3}
          mb={3}
          borderBottom='1px solid GrayText'
        >
          {recentProps.map((prop, index) => (
            <LibraryCard
              cardType='prop'
              index={index}
              name={prop.name}
              image={prop.thumbnail}
              filterOptions={prop.categories}
              selected={selected}
              handleSelectionChange={handleSelectionChange}
            />
          ))}
        </Box>
      </Box>
      <Box
        display='flex'
        flexWrap='wrap'
        gap={1}
        alignItems='start'
        justifyContent='space-evenly'
        marginTop={recentProps.length === 0 ? 3 : 0}
      >
        {props.map((prop, index) => (
          <LibraryCard
            cardType='prop'
            index={index + 6}
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
