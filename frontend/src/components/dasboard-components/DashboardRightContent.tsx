import React from "react";
import { Paper, Box, Typography, Button } from "@mui/material";
import { styled } from "@mui/material/styles";
import { blue } from "@mui/material/colors";

const ColumnPaper = styled(Paper)(({ theme }) => ({
  width: "100%",
  height: "100%",
}));
const ColumnHeader = styled(Box)(({ theme }) => ({
  width: "100%",
  height: "50px",
  borderRadius: "1px 1px 0px 0px",
  backgroundColor: blue[500],
  display: "flex",
  flexDirection: "column",
  justifyContent: "center",
  alignItems: "center",
}));

export default function DashboardRightContent() {
  return (
    <ColumnPaper>
      <ColumnHeader>
        <Typography variant='h6' fontWeight={700} fontSize='14pt'>
          PROJECTS
        </Typography>
      </ColumnHeader>
    </ColumnPaper>
  );
}
