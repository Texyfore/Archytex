import React from "react";

import Box from "@mui/material/Box";

import ProjectBrowser from "./projects-subpage-components/ProjectBrowser";
import SettingBrowser from "./settings-subpage-components/SettingBrowser";
import SubPageContainer from "./SubPageContainer";

export default function DesktopDashboard() {
  return (
    <Box width='100%' height='100%'>
      <SubPageContainer trigger='projects'>
        <ProjectBrowser />
      </SubPageContainer>
      <SubPageContainer trigger='settings'>
        <SettingBrowser />
      </SubPageContainer>
    </Box>
  );
}
