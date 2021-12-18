import React from "react";
import { Paper, Box, Typography, Grow } from "@mui/material";
import { styled } from "@mui/material/styles";
import ProjectBrowser from "./ProjectBrowser";
import SettingsBrowser from "./settings-subpage/SettingsBrowser";
import { useSubPage } from "../../services/selectedDashboardSubPage";

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
  return (
    <ColumnPaper elevation={0}>
      <ColumnHeader>
        <Box height={1.01} width='100%' sx={{ backgroundColor: "#39A0ED" }} />
        <Grow in={subpage === "projects"}>
          <Typography
            variant='h6'
            fontWeight={600}
            fontSize='1em'
            width={100}
            paddingX={2}
            display={subpage === "projects" ? "block" : "none"}
          >
            Projects
          </Typography>
        </Grow>
        <Grow in={subpage === "settings"}>
          <Typography
            variant='h6'
            fontWeight={600}
            fontSize='1em'
            width={100}
            paddingX={2}
            display={subpage === "settings" ? "block" : "none"}
          >
            Settings
          </Typography>
        </Grow>
        <Box height={1.01} width='100%' sx={{ backgroundColor: "#39A0ED" }} />
      </ColumnHeader>
      <Grow in={subpage === "projects"}>
        <Box height='100%' display={subpage === "projects" ? "block" : "none"}>
          <ProjectBrowser />
        </Box>
      </Grow>
      <Grow in={subpage === "settings"}>
        <Box height='100%' display={subpage === "settings" ? "block" : "none"}>
          <SettingsBrowser />
        </Box>
      </Grow>
    </ColumnPaper>
  );
}
