import React from "react";

import Box from "@mui/material/Box";

import SubPageContainer from "./SubPageContainer";
import ProjectBrowser from "./projects-subpage-components/ProjectBrowser";
import SettingBrowser from "./settings-subpage-components/SettingBrowser";

export default function DashboardRightContent() {
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
