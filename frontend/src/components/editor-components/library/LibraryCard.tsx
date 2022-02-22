import React from "react";
import {
  Box,
  Card,
  CardActionArea,
  CardContent,
  CardMedia,
  Chip,
  Typography,
  Tooltip,
} from "@mui/material";

enum TextureFilterOptions {
  brick = "Brick",
  wood = "Wood",
  concrete = "Concrete",
  rock = "Rock",
  dirty = "Dirty",
  clean = "Clean",
}
enum PropFilterOptions {
  furniture = "Furniture",
  decoration = "Decoration",
  table = "Table",
  chair = "Chair",
}
interface LibraryCardProps {
  cardType: "prop" | "texture";
  index: number;
  name: string;
  image: string;
  filterOptions: TextureFilterOptions[] | PropFilterOptions[];
  selected: number | undefined;
  handleSelectionChange: (n: number | undefined) => void;
}

export default function LibraryCard({
  cardType,
  index,
  name,
  image,
  filterOptions,
  selected,
  handleSelectionChange,
}: LibraryCardProps) {
  const click = () => {
    index === selected
      ? handleSelectionChange(undefined)
      : handleSelectionChange(index);
  };

  return (
    <Card
      sx={
        index === selected
          ? {
              width: 175,
              border: "2px solid #39A0ED",
              transition: "border 200ms ease-in-out",
            }
          : { width: 175, border: "2px solid transparent" }
      }
    >
      <CardActionArea onClick={click}>
        <CardMedia
          component='img'
          height='140'
          image={image}
          alt='prop'
          sx={
            cardType === "prop" ? { objectFit: "contain", padding: "10px" } : {}
          }
        />
        <CardContent>
          <Tooltip title={name} placement='top'>
            <Typography gutterBottom width='100%' noWrap>
              {name}
            </Typography>
          </Tooltip>
          <Box display='flex' flexWrap='wrap' justifyContent='start' gap={1}>
            {filterOptions.map((filterOption) => (
              <Chip size='small' label={filterOption} />
            ))}
          </Box>
        </CardContent>
      </CardActionArea>
    </Card>
  );
}
