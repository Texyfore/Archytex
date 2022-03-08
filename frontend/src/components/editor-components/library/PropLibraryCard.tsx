import React from "react";

import Box from "@mui/material/Box";
import Card from "@mui/material/Card";
import CardActionArea from "@mui/material/CardActionArea";
import CardContent from "@mui/material/CardContent";
import CardMedia from "@mui/material/CardMedia";
import Chip from "@mui/material/Chip";
import Tooltip from "@mui/material/Tooltip";
import Typography from "@mui/material/Typography";

import { Prop } from "../../../services/Library";

interface Props {
  prop: Prop;
  isSelected: boolean;
  handleSelectionChange: (item: Prop | undefined) => void;
}

export default function PropLibraryCard({
  prop,
  isSelected,
  handleSelectionChange,
}: Props) {
  const handleClick = () => {
    isSelected ? handleSelectionChange(undefined) : handleSelectionChange(prop);
  };

  return (
    <Card
      sx={
        isSelected
          ? {
              width: 175,
              border: "2px solid #39A0ED",
              transition: "border 200ms ease-in-out",
            }
          : { width: 175, border: "2px solid transparent" }
      }
    >
      <CardActionArea onClick={handleClick}>
        <CardMedia
          component='img'
          height='140'
          image={
            require(`../../../../public/assets/thumbnails/${prop.name}.webp`)
              .default
          }
          alt='item'
          sx={{ objectFit: "contain", padding: "10px" }}
        />
        <CardContent>
          <Tooltip
            title={
              prop.name.charAt(0).toUpperCase() +
              prop.name.replaceAll("_", " ").slice(1)
            }
            placement='top'
          >
            <Typography gutterBottom width='100%' noWrap>
              {prop.name.charAt(0).toUpperCase() +
                prop.name.replaceAll("_", " ").slice(1)}
            </Typography>
          </Tooltip>
          <Box display='flex' flexWrap='wrap' justifyContent='start' gap={1}>
            {prop.public !== null &&
              prop.public.categories.map((category) => (
                <Chip
                  size='small'
                  label={category.charAt(0).toUpperCase() + category.slice(1)}
                />
              ))}
          </Box>
        </CardContent>
      </CardActionArea>
    </Card>
  );
}
