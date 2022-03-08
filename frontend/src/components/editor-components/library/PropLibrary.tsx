import React from "react";

import Box from "@mui/material/Box";

import LibraryCard from "./LibraryCard";
import { Prop } from "../../../services/Library";

interface Props {
  selected: Prop | undefined;
  handleSelectionChange: (prop: Prop | undefined) => void;
  query: string;
  checkedCategories: string[];
  props: Prop[];
}
export default function PropLibrary({
  selected,
  handleSelectionChange,
  query,
  checkedCategories,
  props,
}: Props) {
  const matchesFilter = (prop: Prop) => {
    if (prop.public !== null) {
      return prop.public.categories.some((category) =>
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
      marginTop={3}
    >
      {props
        .filter(
          (p) =>
            p.name.toLowerCase().includes(query.toLowerCase()) &&
            matchesFilter(p)
        )
        .map((prop, index) => (
          <LibraryCard
            key={index}
            cardType='prop'
            item={prop}
            isSelected={
              selected === undefined ? false : selected.id === prop.id
            }
            handleSelectionChange={handleSelectionChange}
          />
        ))}
    </Box>
  );
}
