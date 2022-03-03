import React from "react";

import Box from "@mui/material/Box";
import Card from "@mui/material/Card";
import CardActionArea from "@mui/material/CardActionArea";
import CardContent from "@mui/material/CardContent";
import CardMedia from "@mui/material/CardMedia";
import Chip from "@mui/material/Chip";
import Tooltip from "@mui/material/Tooltip";
import Typography from "@mui/material/Typography";

import Texture from "../../../services/types/Texture";
import Prop from "../../../services/types/Prop";
import Category from "../../../services/libraries/Category";

interface Props {
  cardType: "prop" | "texture";
  item: Texture | Prop;
  isSelected: boolean;
  handleSelectionChange: (item: Texture | Prop | undefined) => void;
}

export default function LibraryCard({
  cardType,
  item,
  isSelected,
  handleSelectionChange,
}: Props) {
  const handleClick = () => {
    isSelected ? handleSelectionChange(undefined) : handleSelectionChange(item);
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
          image={item.thumbnail}
          alt='prop'
          sx={
            cardType === "prop" ? { objectFit: "contain", padding: "10px" } : {}
          }
        />
        <CardContent>
          <Tooltip title={item.name} placement='top'>
            <Typography gutterBottom width='100%' noWrap>
              {item.name}
            </Typography>
          </Tooltip>
          <Box display='flex' flexWrap='wrap' justifyContent='start' gap={1}>
            {item.categories.map((category) => (
              <Chip size='small' label={category.name} />
            ))}
          </Box>
        </CardContent>
      </CardActionArea>
    </Card>
  );
}
