import React from "react";

import AppBarOffset from "../components/app-bar-components/AppBarOffset";
import DashboardContainer from "../components/dashboard-components/DashboardContainer";
import ProjectBrowser from "../components/dashboard-components/ProjectBrowser";

export default function Dashboard() {
  return (
    <>
      <AppBarOffset />
      <DashboardContainer>
        <ProjectBrowser />
      </DashboardContainer>
    </>
  );
}
