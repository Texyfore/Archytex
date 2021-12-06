import React from "react";
import { Paper, Box, Typography } from "@mui/material";
import { styled } from "@mui/material/styles";
import ProjectBrowser from "./ProjectBrowser";
import SettingsBrowser from "./settings-subpage/SettingsBrowser";
import { SubPage, useSubPage } from "../../services/selectedDashboardSubPage";

const ColumnPaper = styled(Paper)(({ theme }) => ({
  width: "100%",
  height: "100%",
  filter: "drop-shadow(0px 0px 4px rgba(0,0,0,0.5))",
}));
const ColumnHeader = styled(Box)(({ theme }) => ({
  width: "100%",
  height: "50px",
  borderRadius: "2px 2px 0px 0px",
  display: "flex",
  justifyContent: "center",
  alignItems: "center",
}));
export default function DashboardRightContent() {
  const [subpage, _] = useSubPage();
  const getSubPage = (subpage: SubPage) => {
    console.log(subpage);
    switch (subpage) {
      case "projects":
        return <ProjectBrowser />;
      case "settings":
        return <SettingsBrowser />;
      default:
        break;
    }
  };

  const getTitle = (subpage: SubPage) => {
    switch (subpage) {
      case "projects":
        return "Projects";

      case "community":
        return "Community portfolio";
      case "settings":
        return "Settings";
      default:
        break;
    }
  };
  return (
    <ColumnPaper elevation={0}>
      <ColumnHeader>
        <Box height={1.01} width='100%' sx={{ backgroundColor: "#39A0ED" }} />
        <Typography variant='h6' fontWeight={600} fontSize='1em' paddingX={2}>
          {getTitle(subpage)}
        </Typography>
        <Box height={1.01} width='100%' sx={{ backgroundColor: "#39A0ED" }} />
      </ColumnHeader>
      {getSubPage(subpage)}
    </ColumnPaper>
  );
}
