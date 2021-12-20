import React from "react";
import { styled, useTheme } from "@mui/material/styles";
import { Grid, useMediaQuery } from "@mui/material";
import DashboardLeftContent from "../components/dashboard-components/DashboardLeftContent";
import DashboardRightContent from "../components/dashboard-components/DashboardRightContent";
import { ProjectsProvider } from "../services/projects";
import { SubPageProvider } from "../services/selectedDashboardSubPage";
import { useApi } from "../services/user/api";
import ArchytexAppBar from "../components/ArchytexAppBar";

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
  useApi(true);
  const theme = useTheme();
  const isContainer = useMediaQuery(theme.breakpoints.up("lg"));

  return (
    <SubPageProvider>
      <React.Fragment>
        <ArchytexAppBar content="dashboard" />
        {/* TODO: Bring back mini drawer */}
        <MaxHeightGrid
          container
          overflow='hidden'
          display='flex'
          direction={{ lg: "column" }}
          key='left'
        >
          <Grid item>
          </Grid>

          <Grid
            item
            container={isContainer}
            component='main'
            columnSpacing={{ xs: 0, lg: 10 }}
            padding={{ lg: 4 }}
            sx={{
              flexGrow: 1,
              backgroundImage: `radial-gradient(#1c517a 0.75px, ${
                theme.palette.mode === "dark" ? "#0c0c0c" : "#F5F0F6"
              } 0.75px)`,
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
              <ProjectsProvider>
                <DashboardRightContent />
              </ProjectsProvider>
            </CalcHeightGridItem>
          </Grid>
        </MaxHeightGrid>
      </React.Fragment>
    </SubPageProvider>
  );
}
