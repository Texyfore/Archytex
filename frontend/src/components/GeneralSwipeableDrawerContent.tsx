import React from "react";
import {
  Box,
  List,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Typography,
} from "@mui/material";
import { styled } from "@mui/material/styles";
import { DashboardRounded, Home, Login } from "@mui/icons-material";
import ArchytexIcon from "./ArchytexIcon";
import LanguageSelectDropdown from "./LanguageSelectDropdown";
import DarkModeSwitch from "./DarkModeSwitch";
import { ColorMode, useColorMode } from "../services/colorMode";
import { useHistory } from "react-router-dom";
import { useApi } from "../services/user/api";

const DrawerHeader = styled("div")(({ theme }) => ({
  display: "flex",
  alignItems: "center",
  justifyContent: "flex-end",
  padding: theme.spacing(0, 1),
  // necessary for content to be below app bar
  ...theme.mixins.toolbar,
}));
interface navButton {
  text: string;
  icon: JSX.Element;
  route: string;
}

export default function GeneralSwipeableDrawerContent() {
  const api = useApi();
  const buttonList: navButton[] =
    api?.state === "logged-in"
      ? [
          {
            text: "Home",
            icon: <Home />,
            route: "/",
          },
          {
            text: "Dashboard",
            icon: <DashboardRounded />,
            route: "/dashboard",
          },
        ]
      : [
          {
            text: "Home",
            icon: <Home />,
            route: "/",
          },
          {
            text: "Login",
            icon: <Login />,
            route: "/login",
          },
        ];
  const [colorMode, _] = useColorMode();

  const history = useHistory();
  const handleClick = (route: string) => {
    history.push(route);
  };

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
      <DrawerHeader
        sx={{
          width: 300,
          height: 150,
          display: "flex",
          justifyContent: "center",
          backgroundSize: "10px 10px",
          backgroundImage:
            colorMode === ColorMode.Dark
              ? "radial-gradient(#1c517a .75px, #0c0c0c .75px)"
              : "radial-gradient(#1c517a .75px, #f5f0f6 .75px)",
        }}
      >
        <ArchytexIcon />
        <Typography variant='h6'>Archytex</Typography>
      </DrawerHeader>
      <List>
        {buttonList.map((props, index) => (
          <ListItemButton
            sx={{
              borderRadius: "2px",
            }}
            key={props.text}
            onClick={() => handleClick(props.route)}
          >
            <ListItemIcon>{props.icon}</ListItemIcon>
            <ListItemText primary={props.text} />
          </ListItemButton>
        ))}
      </List>
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
