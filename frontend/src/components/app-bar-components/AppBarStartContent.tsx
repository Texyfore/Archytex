import React, { useState } from "react";

import { useTheme } from "@mui/material/styles";
import useMediaQuery from "@mui/material/useMediaQuery";
import Box from "@mui/material/Box";
import IconButton from "@mui/material/IconButton";

import { Menu } from "@mui/icons-material";

import LogoWithText from "../general-components/LogoWithText";
import AppBarNavigationButtons from "./AppBarNavigationButtons";
import MainSwipeableDrawer from "../swipeable-drawer-components/MainSwipeableDrawer";

export default function AppBarStartContent() {
  const theme = useTheme();
  const upMd = useMediaQuery(theme.breakpoints.up("md"));

  const [drawerOpen, setDrawerOpen] = useState(false);
  const handleOpenChange = (value: boolean) => {
    setDrawerOpen(value);
  };
  const handleDrawerToggle = () => {
    handleOpenChange(!drawerOpen);
  };

  return (
    <>
      <Box display='flex'>
        {upMd ? (
          <>
            <LogoWithText />
            <AppBarNavigationButtons />
          </>
        ) : (
          <IconButton onClick={handleDrawerToggle}>
            <Menu color='primary' />
          </IconButton>
        )}
      </Box>

      <MainSwipeableDrawer
        open={drawerOpen}
        handleOpenChange={handleDrawerToggle}
      />
    </>
  );
}
