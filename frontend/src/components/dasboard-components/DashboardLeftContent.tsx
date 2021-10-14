import React from "react";
import { Paper, Box } from "@mui/material";
import { styled } from "@mui/material/styles";
import DashboardUserData from "./DashboardUserData";

const ColumnPaper = styled(Paper)(({ theme }) => ({
  width: "100%",
  height: "100%",
  display: "flex",
  flexDirection: "column",
}));

export default function DashboardLeftContent() {
  return (
    <ColumnPaper sx={{ display: "flex" }}>
      <DashboardUserData />
      {/* <DashboardControllerButtons /> */}
    </ColumnPaper>
  );
}
