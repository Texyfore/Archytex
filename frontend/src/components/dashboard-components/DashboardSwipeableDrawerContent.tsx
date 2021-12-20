import React from "react";
import { styled } from "@mui/material/styles";
import DashboardUserData from "./DashboardUserData";
import DashboardControllerButtons from "./DashboardControllerButtons";

const DrawerHeader = styled("div")(({ theme }) => ({
  display: "flex",
  alignItems: "center",
  justifyContent: "flex-end",
  padding: theme.spacing(0, 1),
  // necessary for content to be below app bar
  ...theme.mixins.toolbar,
}));


export default function DashboardSwipeableDrawerContent() {
  return (
    <>
      <DrawerHeader sx={{ width: 300 }} />
      <DashboardUserData />
      <DashboardControllerButtons /></>
  );
}
