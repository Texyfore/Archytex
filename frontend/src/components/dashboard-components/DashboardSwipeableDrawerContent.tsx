import React from "react";
import { Box } from "@mui/material";
import { styled } from "@mui/material/styles";
import DashboardUserData from "./DashboardUserData";
import DashboardControllerButtons from "./DashboardControllerButtons";
import DarkModeSwitch from "../DarkModeSwitch";
import LanguageSelectDropdown from "../LanguageSelectDropdown";
import { useColorMode } from "../../services/colorMode";

const DrawerHeader = styled("div")(({ theme }) => ({
  display: "flex",
  alignItems: "center",
  justifyContent: "flex-end",
  padding: theme.spacing(0, 1),
  // necessary for content to be below app bar
  ...theme.mixins.toolbar,
}));

export default function DashboardSwipeableDrawerContent() {
  const [colorMode, _] = useColorMode();

  const [anchorEl, setAnchorEl] = React.useState<null | HTMLElement>(null);
  const languageMenuOpen = Boolean(anchorEl);
  const handleLanguageMenuClick = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };
  const handleLanguageMenuClose = () => {
    setAnchorEl(null);
  };
  return (
    <>
      <DrawerHeader sx={{ width: 300 }} />
      <DashboardUserData />
      <DashboardControllerButtons />
      <Box
        marginTop='auto'
        marginBottom={2}
        display='flex'
        alignItems='end'
        justifyContent='space-evenly'
      >
        <DarkModeSwitch />
        <LanguageSelectDropdown
          open={languageMenuOpen}
          handleClick={handleLanguageMenuClick}
          handleClose={handleLanguageMenuClose}
          anchorEl={anchorEl}
        />
      </Box>
    </>
  );
}
