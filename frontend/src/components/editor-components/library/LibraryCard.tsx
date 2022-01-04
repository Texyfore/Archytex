import React from "react";
import {
  Card,
  CardActionArea,
  CardContent,
  CardMedia,
  Typography,
} from "@mui/material";
import image1 from "../../../img/1.jpg";

export default function LibraryCard() {
  return (
    <Card sx={{ maxWidth: 160 }}>
      <CardActionArea>
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
