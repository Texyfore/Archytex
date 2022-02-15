import React from "react";

import AppBarOffset from "../components/app-bar-components/AppBarOffset";
import DashboardContainer from "../components/dashboard-components/DashboardContainer";
import DashboardSideBar from "../components/dashboard-components/DashboardSideBar";
import DashboardContent from "../components/dashboard-components/DashboardContent";

export default function Dashboard() {
  return (
    <>
      <AppBarOffset />
      <DashboardContainer>
        <DashboardSideBar />
        <DashboardContent></DashboardContent>
      </DashboardContainer>
    </>
  );
}
