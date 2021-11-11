import { Box, Button, Typography } from "@mui/material";
import React from "react";

export default function MainPage() {
  return (
    <React.Fragment>
      <Box
        height='60vh'
        sx={{
          // background-color: #e5e5f7;
          // opacity: 0.8;
          // background-size: 20px 20px;
          // background-image:  repeating-linear-gradient(to right, #444cf7, #444cf7 1px, #e5e5f7 1px, #e5e5f7);
          backgroundColor: "#e5e5f7",
          opacity: 1,
          backgroundSize: "40px 40px",
          backgroundImage:
            "repeating-linear-gradient(to right, #444cf7, #444cf7 1px, #e5e5f7 1px, #e5e5f7)",
        }}
      >
        <Typography variant='h1' textAlign='center'>
          Archytex
        </Typography>
        <Typography variant='subtitle1' textAlign='center'>
          Take your architectural visualisations to the next level
        </Typography>
        <Button variant='outlined' size='large'>
          Learn more{" "}
        </Button>
      </Box>
      <Box height='5000px'>asd</Box>
    </React.Fragment>
  );
}
