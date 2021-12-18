import React from "react";
import {
  Box,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  SwipeableDrawer,
  Typography,
} from "@mui/material";
import { styled } from "@mui/material/styles";
import { Home, Login, People } from "@mui/icons-material";
import GeneralSwipeableDrawerContent from "./GeneralSwipeableDrawerContent";
import DashboardSwipeableDrawerContent from "./dashboard-components/DashboardSwipeableDrawerContent";

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
  content: "general" | "dashboard";
}
const buttonList = [
  {
    text: "Home",
    icon: <Home />,
  },
  {
    text: "Community",
    icon: <People />,
  },
  {
    text: "Login",
    icon: <Login />,
  },
];

const getDrawerContent = (content: "general" | "dashboard") => {
  switch (content) {
    case "general":
      return <GeneralSwipeableDrawerContent />
    case "dashboard":
      return <DashboardSwipeableDrawerContent />
    default:
      return;
  }
}

export default function DashboardSwipeableDrawer({
  open,
  handleOpenChange,
  content
}: SwipeableDrawerProps) {
  const [anchorEl, setAnchorEl] = React.useState<null | HTMLElement>(null);
  const languageMenuOpen = Boolean(anchorEl);
  const handleLanguageMenuClick = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };
  const handleLanguageMenuClose = () => {
    setAnchorEl(null);
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
      {getDrawerContent(content)}
    </SwipeableDrawer>
  );
}
