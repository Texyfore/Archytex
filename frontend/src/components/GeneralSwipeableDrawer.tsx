import React from "react";
import {
  SwipeableDrawer,
} from "@mui/material";
import GeneralSwipeableDrawerContent from "./GeneralSwipeableDrawerContent";
import DashboardSwipeableDrawerContent from "./dashboard-components/DashboardSwipeableDrawerContent";

interface SwipeableDrawerProps {
  open: boolean;
  handleOpenChange: (value: boolean) => void;
  content: "general" | "dashboard";
}

const getDrawerContent = (content: "general" | "dashboard") => {
  switch (content) {
    case "general":
      return <GeneralSwipeableDrawerContent />
    case "dashboard":
      return <DashboardSwipeableDrawerContent />
    default:
      return;
  }
}

export default function DashboardSwipeableDrawer({
  open,
  handleOpenChange,
  content
}: SwipeableDrawerProps) {

  return (
    <SwipeableDrawer
      sx={{ display: { xs: "flex", md: "none" } }}
      anchor='left'
      open={open}
      elevation={0}
      onClose={() => handleOpenChange(false)}
      onOpen={() => handleOpenChange(true)}
    >
      {getDrawerContent(content)}
    </SwipeableDrawer>
  );
}
