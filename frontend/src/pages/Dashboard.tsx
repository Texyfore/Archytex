import React from "react";

import AppBarOffset from "../components/app-bar-components/AppBarOffset";
import DashboardContainer from "../components/dashboard-components/DashboardContainer";
import DashboardSideBar from "../components/dashboard-components/DashboardSideBar";
import DashboardContent from "../components/dashboard-components/DashboardContent";

import { SubPageProvider } from "../services/selectedDashboardSubPage";

export default function Dashboard() {
  return (
    <>
      <AppBarOffset />
      <SubPageProvider>
        <DashboardContainer>
          <DashboardSideBar />
          <DashboardContent />
        </DashboardContainer>
      </SubPageProvider>
    </>
  );
}
