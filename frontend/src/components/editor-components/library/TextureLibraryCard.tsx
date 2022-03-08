import React from "react";

import Box from "@mui/material/Box";
import Card from "@mui/material/Card";
import CardActionArea from "@mui/material/CardActionArea";
import CardContent from "@mui/material/CardContent";
import CardMedia from "@mui/material/CardMedia";
import Chip from "@mui/material/Chip";
import Tooltip from "@mui/material/Tooltip";
import Typography from "@mui/material/Typography";

import { Texture } from "../../../services/Library";

interface Props {
  texture: Texture;
  isSelected: boolean;
  handleSelectionChange: (item: Texture | undefined) => void;
}

export default function TextureLibraryCard({
  texture,
  isSelected,
  handleSelectionChange,
}: Props) {
  const handleClick = () => {
    isSelected
      ? handleSelectionChange(undefined)
      : handleSelectionChange(texture);
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
            require(`../../../../public/assets/thumbnails/${texture.name}.webp`)
              .default
          }
          alt='item'
        />
        <CardContent>
          <Tooltip
            title={
              texture.name.charAt(0).toUpperCase() +
              texture.name.replaceAll("_", " ").slice(1)
            }
            placement='top'
          >
            <Typography gutterBottom width='100%' noWrap>
              {texture.name.charAt(0).toUpperCase() +
                texture.name.replaceAll("_", " ").slice(1)}
            </Typography>
          </Tooltip>
          <Box display='flex' flexWrap='wrap' justifyContent='start' gap={1}>
            {texture.public !== null &&
              texture.public.categories.map((category) => (
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
