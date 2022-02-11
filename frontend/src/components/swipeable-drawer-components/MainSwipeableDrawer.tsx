import React from "react";

import SwipeableDrawer from "@mui/material/SwipeableDrawer";
import DrawerContent from "./DrawerContent";

interface Props {
  open: boolean;
  handleOpenChange: (value: boolean) => void;
}

export default function MainSwipeableDrawer({ open, handleOpenChange }: Props) {
  return (
    <SwipeableDrawer
      sx={{ display: { xs: "flex", md: "none" } }}
      anchor='left'
      open={open}
      elevation={0}
      onClose={() => handleOpenChange(false)}
      onOpen={() => handleOpenChange(true)}
    >
      <DrawerContent />
    </SwipeableDrawer>
  );
}
