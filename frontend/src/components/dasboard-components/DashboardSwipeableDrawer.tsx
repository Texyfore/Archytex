import React from "react";
import { SwipeableDrawer } from "@mui/material";
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

interface SwipeableDrawerProps {
  open: boolean;
  handleOpenChange: (value: boolean) => void;
}

export default function DashboardSwipeableDrawer({
  open,
  handleOpenChange,
}: SwipeableDrawerProps) {
  return (
    <SwipeableDrawer
      sx={{ display: { xs: "flex", md: "none" } }}
      anchor='left'
      open={open}
      onClose={() => handleOpenChange(false)}
      onOpen={() => handleOpenChange(true)}
    >
      <DrawerHeader sx={{ width: 300 }} />
      <DashboardUserData />
      <DashboardControllerButtons />
    </SwipeableDrawer>
  );
}
