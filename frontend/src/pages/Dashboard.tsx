import React, { useState } from "react";
import { styled } from "@mui/material/styles";
import Box from "@mui/material/Box";
import { Grid } from "@mui/material";
import DashboardAppBar from "../components/dasboard-components/DashboardAppBar";
import DashboardMiniDrawer from "../components/dasboard-components/DashboardMiniDrawer";
import DashboardSwipeableDrawer from "../components/dasboard-components/DashboardSwipeableDrawer";
import DashboardLeftContent from "../components/dasboard-components/DashboardLeftContent";
import DashboardRightContent from "../components/dasboard-components/DashboardRightContent";

const Offset = styled("div")(({ theme }) => ({
  padding: theme.spacing(0, 1),
  // necessary for content to be below app bar
  ...theme.mixins.toolbar,
}));

const MaxHeightGrid = styled(Grid)(({ theme }) => ({
  marginTop: 56,
  height: `calc(100vh - 56px)`,
  [`${theme.breakpoints.up("xs")} and (orientation: landscape)`]: {
    marginTop: 48,
    height: `calc(100vh - 48px)`,
  },
  [theme.breakpoints.up("sm")]: {
    marginTop: 64,
    height: `calc(100vh - 64px)`,
  },
}));

const MyGridItem = styled(Grid)(({ theme }) => ({
  height: `calc(100vh - 56px)`,
  [`${theme.breakpoints.up("xs")} and (orientation: landscape)`]: {
    height: `calc(100vh - 48px)`,
  },
  [theme.breakpoints.up("lg")]: {
    height: `calc(100vh - 64px - 2*(${theme.spacing(4)}))`,
  },
}));

export default function Dashboard() {
  const [open, setOpen] = useState(false);

  function handleOpenChange(value: boolean): void {
    setOpen(value);
  }

  return (
    <MaxHeightGrid container direction='column' display='flex' key='left'>
      <DashboardAppBar open={open} handleOpenChange={handleOpenChange} />

      <DashboardSwipeableDrawer
        open={open}
        handleOpenChange={handleOpenChange}
      />

      <DashboardMiniDrawer open={open} handleOpenChange={handleOpenChange} />

      <Grid
        item
        container
        component='main'
        sx={{ flexGrow: 1 }}
        columnSpacing={{ xs: 0, lg: 10 }}
        padding={{ lg: 4 }}
      >
        <MyGridItem item xs={12} md={4} display={{ xs: "none", lg: "flex" }}>
          <DashboardLeftContent />
        </MyGridItem>
        <MyGridItem item xs={12} lg={8}>
          <DashboardRightContent />
        </MyGridItem>
      </Grid>
    </MaxHeightGrid>
  );
}
