import React, { useState } from "react";
import { styled, useTheme } from "@mui/material/styles";
import { Grid, useMediaQuery } from "@mui/material";
import DashboardAppBar from "../components/dasboard-components/DashboardAppBar";
import DashboardMiniDrawer from "../components/dasboard-components/DashboardMiniDrawer";
import DashboardSwipeableDrawer from "../components/dasboard-components/DashboardSwipeableDrawer";
import DashboardLeftContent from "../components/dasboard-components/DashboardLeftContent";
import DashboardRightContent from "../components/dasboard-components/DashboardRightContent";

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

const CalcHeightGridItem = styled(Grid)(({ theme }) => ({
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

  const theme = useTheme();
  const isContainer = useMediaQuery(theme.breakpoints.up("lg"));

  function handleOpenChange(value: boolean): void {
    setOpen(value);
  }

  return (
    // TODO: pattern.css
    <React.Fragment>
      <DashboardAppBar open={open} handleOpenChange={handleOpenChange} />

      <DashboardSwipeableDrawer
        open={open}
        handleOpenChange={handleOpenChange}
      />

      <MaxHeightGrid
        container
        overflow='hidden'
        display='flex'
        direction={{ lg: "column" }}
        key='left'
      >
        <Grid item>
          <DashboardMiniDrawer
            open={open}
            handleOpenChange={handleOpenChange}
          />
        </Grid>

        <Grid
          item
          container={isContainer}
          component='main'
          columnSpacing={{ xs: 0, lg: 10 }}
          padding={{ lg: 4 }}
          sx={{
            flexGrow: 1,
            backgroundImage: "radial-gradient(#1c517a 0.75px, #0c0c0c 0.75px)",
            backgroundSize: "15px 15px",
          }}
        >
          <CalcHeightGridItem
            item
            xs={12}
            md={4}
            display={{ xs: "none", lg: "flex" }}
          >
            <DashboardLeftContent />
          </CalcHeightGridItem>
          <CalcHeightGridItem item xs={12} lg={8}>
            <DashboardRightContent />
          </CalcHeightGridItem>
        </Grid>
      </MaxHeightGrid>
    </React.Fragment>
  );
}
