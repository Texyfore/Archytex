import React from "react";
import { Close, MenuOutlined } from "@mui/icons-material";
import { Box, IconButton } from "@mui/material";
import MuiDrawer from "@mui/material/Drawer";
import { styled, Theme, CSSObject } from "@mui/material/styles";
import DashboardUserData from "./DashboardUserData";
import DashboardControllerButtons from "./DashboardControllerButtons";

const drawerWidth = 300;

interface MiniDrawerProps {
  open: boolean;
  handleOpenChange: (value: boolean) => void;
}

const DrawerHeader = styled("div")(({ theme }) => ({
  display: "flex",
  alignItems: "center",
  justifyContent: "flex-end",
  padding: theme.spacing(0, 1),
  // necessary for content to be below app bar
  ...theme.mixins.toolbar,
}));

const openedMixin = (theme: Theme): CSSObject => ({
  width: drawerWidth,
  transition: theme.transitions.create("width", {
    easing: theme.transitions.easing.sharp,
    duration: theme.transitions.duration.enteringScreen,
  }),
  overflowX: "hidden",
});

const closedMixin = (theme: Theme): CSSObject => ({
  transition: theme.transitions.create("width", {
    easing: theme.transitions.easing.sharp,
    duration: theme.transitions.duration.leavingScreen,
  }),
  overflowX: "hidden",
  width: `calc(${theme.spacing(8)} + 1px)`,
  [theme.breakpoints.up("sm")]: {
    width: `calc(${theme.spacing(7)} + 1px)`,
  },
});

const Drawer = styled(MuiDrawer, {
  shouldForwardProp: (prop) => prop !== "open",
})(({ theme, open }) => ({
  width: drawerWidth,
  flexShrink: 0,
  whiteSpace: "nowrap",
  // boxSizing: "content-box",
  ...theme.mixins.toolbar,
  ...(open && {
    ...openedMixin(theme),
    "& .MuiDrawer-paper": openedMixin(theme),
  }),
  ...(!open && {
    ...closedMixin(theme),
    "& .MuiDrawer-paper": closedMixin(theme),
  }),
}));

export default function DashboardMiniDrawer({
  open,
  handleOpenChange,
}: MiniDrawerProps) {
  // const [open, setOpen] = React.useState(false);

  const toggleDrawerOpen = () => {
    handleOpenChange(!open);
  };

  return (
    <Drawer
      variant='permanent'
      open={open}
      sx={{
        display: { xs: "none", md: "flex", lg: "none" },
        // filter: "drop-shadow(0px 2px 8px rgba(0,0,0,0.5))",
      }}
      PaperProps={{
        sx: {
          border: "none",
          filter: "drop-shadow(4px 0px 4px rgba(0,0,0,0.5))",
        },
      }}
    >
      <DrawerHeader />

      <DrawerHeader>
        <IconButton onClick={toggleDrawerOpen}>
          {open ? <Close /> : <MenuOutlined />}
        </IconButton>
      </DrawerHeader>

      <Box width='100%' display={open ? "block" : "none"}>
        <DashboardUserData />
      </Box>

      <DashboardControllerButtons />
    </Drawer>
  );
}
