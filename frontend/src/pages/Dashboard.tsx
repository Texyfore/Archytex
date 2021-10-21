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

export default function Dashboard() {
  const [open, setOpen] = useState(false);

  function handleOpenChange(value: boolean): void {
    setOpen(value);
  }

  return (
    <Box display='flex' key='left' height={500}>
      <DashboardAppBar open={open} handleOpenChange={handleOpenChange} />

      <DashboardSwipeableDrawer
        open={open}
        handleOpenChange={handleOpenChange}
      />

      <DashboardMiniDrawer open={open} handleOpenChange={handleOpenChange} />

      <Box component='main' sx={{ flexGrow: 1 }}>
        <Offset />
        <Grid
          container
          //TODO: Fix spacing
          columnSpacing={{ xs: 0, md: 0, lg: 10 }}
          paddingTop={{ xs: 0, lg: 4 }}
          paddingLeft={{ xs: 0, lg: 4 }}
          paddingRight={{ xs: 0, lg: 4 }}
          height='88vh'
        >
          <Grid item xs={12} md={4} display={{ xs: "none", lg: "flex" }}>
            <DashboardLeftContent />
          </Grid>
          <Grid item xs={12} lg={8}>
            <DashboardRightContent />
          </Grid>
        </Grid>
      </Box>
    </Box>
  );
}
