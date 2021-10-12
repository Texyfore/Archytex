import * as React from "react";
import { styled, useTheme, Theme, CSSObject } from "@mui/material/styles";
import Box from "@mui/material/Box";
import MuiDrawer from "@mui/material/Drawer";
import MuiAppBar, { AppBarProps as MuiAppBarProps } from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import List from "@mui/material/List";
import CssBaseline from "@mui/material/CssBaseline";
import Typography from "@mui/material/Typography";
import Divider from "@mui/material/Divider";
import IconButton from "@mui/material/IconButton";
import MenuIcon from "@mui/icons-material/Menu";
import ListItem from "@mui/material/ListItem";
import ListItemIcon from "@mui/material/ListItemIcon";
import ListItemText from "@mui/material/ListItemText";
import InboxIcon from "@mui/icons-material/MoveToInbox";
import MailIcon from "@mui/icons-material/Mail";
import { Avatar, Grid, SwipeableDrawer } from "@mui/material";
import ArchytexIcon from "../components/ArchytexIcon";
import { blue } from "@mui/material/colors";
import { BoltOutlined, Close } from "@mui/icons-material";

const drawerWidth = 300;
const swipeableDrawerWidth = "100%";

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

const DrawerHeader = styled("div")(({ theme }) => ({
  display: "flex",
  alignItems: "center",
  justifyContent: "flex-end",
  padding: theme.spacing(0, 1),
  // necessary for content to be below app bar
  ...theme.mixins.toolbar,
}));

interface AppBarProps extends MuiAppBarProps {
  open?: boolean;
}

const AppBar = styled(MuiAppBar, {
  shouldForwardProp: (prop) => prop !== "open",
})<AppBarProps>(({ theme, open }) => ({
  zIndex: theme.zIndex.drawer + 1,
  transition: theme.transitions.create(["width", "margin"], {
    easing: theme.transitions.easing.sharp,
    duration: theme.transitions.duration.leavingScreen,
  }),
  ...(open && {
    marginLeft: drawerWidth,
    width: `calc(100% - ${drawerWidth}px)`,
    transition: theme.transitions.create(["width", "margin"], {
      easing: theme.transitions.easing.sharp,
      duration: theme.transitions.duration.enteringScreen,
    }),
  }),
}));

const Drawer = styled(MuiDrawer, {
  shouldForwardProp: (prop) => prop !== "open",
})(({ theme, open }) => ({
  width: drawerWidth,
  flexShrink: 0,
  whiteSpace: "nowrap",
  boxSizing: "border-box",
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

type Anchor = "top" | "left" | "bottom" | "right";

export default function Dashboard() {
  const theme = useTheme();
  const [open, setOpen] = React.useState(false);
  const [state, setState] = React.useState({
    top: false,
    left: false,
    bottom: false,
    right: false,
  });

  const handleDrawerOpen = () => {
    setOpen(true);
  };

  const handleDrawerClose = () => {
    setOpen(false);
  };

  const toggleSwipeableDrawer =
    (anchor: Anchor, open: boolean) =>
    (event: React.KeyboardEvent | React.MouseEvent) => {
      if (
        event &&
        event.type === "keydown" &&
        ((event as React.KeyboardEvent).key === "Tab" ||
          (event as React.KeyboardEvent).key === "Shift")
      ) {
        return;
      }

      setState({ ...state, [anchor]: open });
    };

  return (
    <React.Fragment key='left'>
      <Box display='flex'>
        <CssBaseline />
        <AppBar position='fixed'>
          <Toolbar sx={{ justifyContent: "space-between" }}>
            <Box display={{ xs: "flex", md: "none" }}>
              <IconButton onClick={open ? handleDrawerClose : handleDrawerOpen}>
                {open ? <Close /> : <MenuIcon />}
              </IconButton>
            </Box>
            <Box display={{ xs: "none", md: "flex" }}>
              <ArchytexIcon />
              <Typography
                variant='h6'
                component='div'
                sx={{ display: { xs: "none", sm: "block" } }}
              >
                ARCHYTEX
              </Typography>
            </Box>
            <Typography
              variant='h6'
              component='div'
              sx={{
                display: { xs: "none", sm: "block" },
              }}
            >
              DASHBOARD
            </Typography>
            <Box sx={{ display: "flex" }}>
              <Typography
                variant='h6'
                component='div'
                sx={{ display: { xs: "none", sm: "block" } }}
              >
                Test User
              </Typography>
              <Avatar sx={{ backgroundColor: "#39A0ED", marginLeft: 2 }} />
            </Box>
          </Toolbar>
        </AppBar>

        <SwipeableDrawer
          anchor='left'
          open={open}
          onClose={toggleSwipeableDrawer("left", false)}
          onOpen={toggleSwipeableDrawer("left", true)}
          sx={{ display: { xs: "flex", md: "none" } }}
        >
          <DrawerHeader sx={{ width: 300 }} />
          <Box
            width={swipeableDrawerWidth}
            display={open ? "flex" : "none"}
            flexDirection='column'
            justifyContent='center'
            gap={1}
            marginTop={4}
          >
            <Avatar
              sx={{
                bgcolor: blue[500],
                color: "white",
                width: "2em",
                height: "2em",
                alignSelf: "center",
                fontSize: "30pt",
              }}
            />
            <Typography variant='h5' textAlign='center'>
              Test User
            </Typography>
            <Box display='flex' justifyContent='center'>
              <BoltOutlined />
              <Typography noWrap>1003</Typography>
            </Box>
          </Box>
          <List>
            {["Inbox", "Starred", "Send email", "Drafts"].map((text, index) => (
              <ListItem button key={text}>
                <ListItemIcon>
                  {index % 2 === 0 ? <InboxIcon /> : <MailIcon />}
                </ListItemIcon>
                <ListItemText primary={text} />
              </ListItem>
            ))}
          </List>
          <Divider />
          <List>
            {["All mail", "Trash", "Spam"].map((text, index) => (
              <ListItem button key={text}>
                <ListItemIcon>
                  {index % 2 === 0 ? <InboxIcon /> : <MailIcon />}
                </ListItemIcon>
                <ListItemText primary={text} />
              </ListItem>
            ))}
          </List>
        </SwipeableDrawer>

        <Drawer
          variant='permanent'
          open={open}
          sx={{ display: { xs: "none", md: "flex", lg: "none" } }}
        >
          <DrawerHeader />
          <DrawerHeader>
            <IconButton onClick={open ? handleDrawerClose : handleDrawerOpen}>
              {open ? <Close /> : <MenuIcon />}
            </IconButton>
          </DrawerHeader>
          <Box
            display={open ? "flex" : "none"}
            flexDirection='column'
            justifyContent='center'
            gap={1}
          >
            <Avatar
              sx={{
                bgcolor: blue[500],
                color: "white",
                width: "2em",
                height: "2em",
                alignSelf: "center",
                fontSize: "30pt",
              }}
            />
            <Typography variant='h5' textAlign='center'>
              Test User
            </Typography>
            <Box display='flex' justifyContent='center'>
              <BoltOutlined />
              <Typography noWrap>1003</Typography>
            </Box>
          </Box>
          <Divider />
          <List>
            {["Inbox", "Starred", "Send email", "Drafts"].map((text, index) => (
              <ListItem button key={text}>
                <ListItemIcon>
                  {index % 2 === 0 ? <InboxIcon /> : <MailIcon />}
                </ListItemIcon>
                <ListItemText primary={text} />
              </ListItem>
            ))}
          </List>
          <Divider />
          <List>
            {["All mail", "Trash", "Spam"].map((text, index) => (
              <ListItem button key={text}>
                <ListItemIcon>
                  {index % 2 === 0 ? <InboxIcon /> : <MailIcon />}
                </ListItemIcon>
                <ListItemText primary={text} />
              </ListItem>
            ))}
          </List>
        </Drawer>

        <Box component='main' sx={{ flexGrow: 1, p: 3 }}>
          <DrawerHeader />
          <Grid
            container
            spacing={{ xs: 0, md: 2, lg: 10 }}
            padding={{ xs: 0, lg: 10 }}
          >
            <Grid item xs={12} md={4} display={{ xs: "none", lg: "flex" }}>
              <Typography paragraph>
                LEFT COLUMN TEXT Lorem ipsum dolor sit amet, consectetur
                adipiscing elit, sed do eiusmod tempor incididunt ut labore et
                dolore magna aliqua. Rhoncus dolor purus non enim praesent
                elementum facilisis leo vel. Risus at ultrices mi tempus
                imperdiet. Semper risus in hendrerit gravida rutrum quisque non
                tellus. Convallis convallis tellus id interdum velit laoreet id
                donec ultrices. Odio morbi quis commodo odio aenean sed
                adipiscing. Amet nisl suscipit adipiscing bibendum est ultricies
                integer quis. Cursus euismod quis viverra nibh cras. Metus
                vulputate eu scelerisque felis imperdiet proin fermentum leo.
                Mauris commodo quis imperdiet massa tincidunt. Cras tincidunt
                lobortis feugiat vivamus at augue. At augue eget arcu dictum
                varius duis at consectetur lorem. Velit sed ullamcorper morbi
                tincidunt. Lorem donec massa sapien faucibus et molestie ac.
              </Typography>
            </Grid>
            <Grid item xs={12} lg={8}>
              <Typography paragraph>
                RIGHT COLUMN TEXT Consequat mauris nunc congue nisi vitae
                suscipit. Fringilla est ullamcorper eget nulla facilisi etiam
                dignissim diam. Pulvinar elementum integer enim neque volutpat
                ac tincidunt. Ornare suspendisse sed nisi lacus sed viverra
                tellus. Purus sit amet volutpat consequat mauris. Elementum eu
                facilisis sed odio morbi. Euismod lacinia at quis risus sed
                vulputate odio. Morbi tincidunt ornare massa eget egestas purus
                viverra accumsan in. In hendrerit gravida rutrum quisque non
                tellus orci ac. Pellentesque nec nam aliquam sem et tortor.
                Habitant morbi tristique senectus et. Adipiscing elit duis
                tristique sollicitudin nibh sit. Ornare aenean euismod elementum
                nisi quis eleifend. Commodo viverra maecenas accumsan lacus vel
                facilisis. Nulla posuere sollicitudin aliquam ultrices sagittis
                orci a.
              </Typography>
            </Grid>
          </Grid>
          {/* <img src={image} height={400} width={400} alt='ARCHYTEX logo' /> */}
        </Box>
      </Box>
    </React.Fragment>
  );
}
