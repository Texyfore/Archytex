import React from "react";
import { Paper } from "@mui/material";
import { styled } from "@mui/material/styles";
import DashboardUserData from "./DashboardUserData";
import DashboardControllerButtons from "./DashboardControllerButtons";

const ColumnPaper = styled(Paper)(({ theme }) => ({
  width: "100%",
  height: "100%",
  display: "flex",
  flexDirection: "column",
  justifyContent: "start",
  borderRadius: 2,
  filter: "drop-shadow(0px 0px 4px rgba(0,0,0,0.5))",
}));

export default function DashboardLeftContent() {
  return (
    <ColumnPaper elevation={0}>
      <DashboardUserData />
      <DashboardControllerButtons />
    </ColumnPaper>
  );
}
