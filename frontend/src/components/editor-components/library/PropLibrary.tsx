import React from "react";
import { Box, Typography } from "@mui/material";
import LibraryCard from "./LibraryCard";
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
import { useTranslation } from "react-i18next";
interface PropLibraryProps {
  selected: number | undefined;
  handleSelectionChange: (n: number | undefined) => void;
}

enum PropFilterOptions {
  furniture = "Furniture",
  decoration = "Decoration",
  table = "Table",
  chair = "Chair",
}

interface Prop {
  name: string;
  filterOptions: PropFilterOptions[];
  image: string;
}

export default function PropLibrary({
  selected,
  handleSelectionChange,
}: PropLibraryProps) {
  const { t } = useTranslation();

  const props: Prop[] = [
    {
      name: "Alarm clock",
      filterOptions: [PropFilterOptions.decoration],
      image: alarmClock,
    },
    {
      name: "Ceramic vase",
      filterOptions: [PropFilterOptions.decoration],
      image: ceramicVase,
    },
    {
      name: "Round coffee table",
      filterOptions: [PropFilterOptions.furniture, PropFilterOptions.table],
      image: roundCoffeeTable,
    },
    {
      name: "Horse statue",
      filterOptions: [PropFilterOptions.decoration],
      image: horseStatue,
    },
    {
      name: "Modern coffee table",
      filterOptions: [PropFilterOptions.furniture, PropFilterOptions.table],
      image: modernCoffeeTable,
    },
    {
      name: "Ottoman",
      filterOptions: [PropFilterOptions.furniture, PropFilterOptions.chair],
      image: ottoman,
    },
    {
      name: "Painted wooden chair",
      filterOptions: [PropFilterOptions.furniture, PropFilterOptions.chair],
      image: paintedWoodenChair,
    },
    {
      name: "Potted plant",
      filterOptions: [PropFilterOptions.decoration],
      image: pottedPlant,
    },
    {
      name: "Standing picture frame",
      filterOptions: [PropFilterOptions.decoration],
      image: standingPictureFrame,
    },
    {
      name: "Steel frame shelves",
      filterOptions: [PropFilterOptions.furniture],
      image: steelFramShelves,
    },
  ];
  const recentProps: Prop[] = [
    // {
    //   name: "Alarm clock",
    //   filterOptions: [PropFilterOptions.decoration],
    //   image: alarmClock,
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
          paddingBottom={3}
          marginBottom={3}
          borderBottom='1px solid GrayText'
        >
          {recentProps.map((prop, index) => (
            <LibraryCard
              cardType='prop'
              index={index}
              name={prop.name}
              image={prop.image}
              filterOptions={prop.filterOptions}
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
            image={prop.image}
            filterOptions={prop.filterOptions}
            selected={selected}
            handleSelectionChange={handleSelectionChange}
          />
        ))}
      </Box>
    </>
  );
}
