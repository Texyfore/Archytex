import React from "react";
import {
  Card,
  CardActionArea,
  CardContent,
  CardMedia,
  Typography,
} from "@mui/material";
import image1 from "../../../img/1.jpg";

interface LibraryCardProps {
  index: number;
  selected: number | undefined;
  handleSelection: (n: number | undefined) => void;
}
export default function LibraryCard({
  index,
  selected,
  handleSelection,
}: LibraryCardProps) {
  const click = () => {
    index === selected ? handleSelection(undefined) : handleSelection(index);
  };
  return (
    <Card
      sx={
        index === selected
          ? {
              maxWidth: 160,
              border: "2px solid #39A0ED",
              transition: "border 200ms ease-in-out",
            }
          : { maxWidth: 160, border: "2px solid transparent" }
      }
    >
      <CardActionArea onClick={click}>
        <CardMedia component='img' height='140' image={image1} alt='texture' />
        <CardContent>
          <Typography gutterBottom component='div'>
            Texture
          </Typography>
        </CardContent>
      </CardActionArea>
    </Card>
  );
}
