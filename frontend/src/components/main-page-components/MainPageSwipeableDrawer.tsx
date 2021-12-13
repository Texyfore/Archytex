import React from "react";
import {
  Box,
  List,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  SwipeableDrawer,
  Typography,
} from "@mui/material";
import { styled } from "@mui/material/styles";
import { Home, Login, People } from "@mui/icons-material";
import ArchytexIcon from "../ArchytexIcon";
import LanguageSelectDropdown from "../LanguageSelectDropdown";
import DarkModeSwitch from "../DarkModeSwitch";
import { ColorMode, useColorMode } from "../../services/colorMode";
import { useHistory } from "react-router-dom";

const DrawerHeader = styled("div")(({ theme }) => ({
  display: "flex",
  alignItems: "center",
  justifyContent: "flex-end",
  padding: theme.spacing(0, 1),
  // necessary for content to be below app bar
  ...theme.mixins.toolbar,
}));

interface SwipeableDrawerProps {
  open: boolean;
  handleOpenChange: (value: boolean) => void;
}
interface navButton {
  text: string;
  icon: JSX.Element;
  route: string;
}
const buttonList: navButton[] = [
  {
    text: "Home",
    icon: <Home />,
    route: "/",
  },
  {
    text: "Community",
    icon: <People />,
    route: "/community",
  },
  {
    text: "Login",
    icon: <Login />,
    route: "/login",
  },
];

export default function DashboardSwipeableDrawer({
  open,
  handleOpenChange,
}: SwipeableDrawerProps) {
  const [anchorEl, setAnchorEl] = React.useState<null | HTMLElement>(null);
  const languageMenuOpen = Boolean(anchorEl);
  const handleLanguageMenuClick = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };
  const handleLanguageMenuClose = () => {
    setAnchorEl(null);
  };
  const [colorMode, _] = useColorMode();

  const history = useHistory();
  const handleClick = (route: string) => {
    history.push(route);
  };
  return (
    <SwipeableDrawer
      sx={{ display: { xs: "flex", md: "none" } }}
      anchor='left'
      open={open}
      elevation={0}
      onClose={() => handleOpenChange(false)}
      onOpen={() => handleOpenChange(true)}
    >
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
    </SwipeableDrawer>
  );
}
