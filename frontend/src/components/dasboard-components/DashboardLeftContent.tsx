import React from "react";
import { Paper } from "@mui/material";
import { styled } from "@mui/material/styles";
import DashboardUserData from "./DashboardUserData";
import DashboardControllerButtons from "./DashboardControllerButtons";
import DashboardLogOut from "./DashboardLogOut";

const ColumnPaper = styled(Paper)(({ theme }) => ({
  width: "100%",
  height: "100%",
  display: "flex",
  flexDirection: "column",
  justifyContent: "space-between",
}));

export default function DashboardLeftContent() {
  return (
    <ColumnPaper>
      <DashboardUserData />
      <DashboardControllerButtons />
      <DashboardLogOut />
    </ColumnPaper>
  );
}
