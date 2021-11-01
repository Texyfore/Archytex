import React from "react";
import { Paper, Box, Typography } from "@mui/material";
import { styled } from "@mui/material/styles";
import ProjectBrowser from "./ProjectBrowser";

const ColumnPaper = styled(Paper)(({ theme }) => ({
  width: "100%",
  height: "100%",
}));
const ColumnHeader = styled(Box)(({ theme }) => ({
  width: "100%",
  height: "50px",
  borderRadius: "1px 1px 0px 0px",
  display: "flex",
  justifyContent: "center",
  alignItems: "center",
}));

export default function DashboardRightContent() {
  return (
    <ColumnPaper elevation={0}>
      <ColumnHeader>
        <Box height={1.01} width='100%' sx={{ backgroundColor: "#39A0ED" }} />
        <Typography variant='h6' fontWeight={600} fontSize='1em' paddingX={2}>
          PROJECTS
        </Typography>
        <Box height={1.01} width='100%' sx={{ backgroundColor: "#39A0ED" }} />
      </ColumnHeader>
      <ProjectBrowser />
    </ColumnPaper>
  );
}
