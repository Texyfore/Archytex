import React from "react";
import {
  AccountCircle,
  Close,
  CreditCard,
  Logout,
  MenuOutlined,
} from "@mui/icons-material";
import {
  AppBar,
  Avatar,
  IconButton,
  Toolbar,
  Typography,
  Box,
  Tooltip,
  Button,
  Menu,
  MenuItem,
  ListItemIcon,
  Divider,
} from "@mui/material";
import ArchytexIcon from "../ArchytexIcon";
import { styled } from "@mui/material/styles";
import DarkModeSwitch from "../DarkModeSwitch";
import { useApi } from "../../services/user/api";

const CustomAppBar = styled(AppBar)(({ theme }) => ({
  zIndex: theme.zIndex.drawer + 1,
  filter: "drop-shadow(0px 2px 4px rgba(0,0,0,0.5))",
}));

interface AppBarProps {
  open: boolean;
  handleOpenChange: (value: boolean) => void;
}

function DashboardAppBar({ open, handleOpenChange }: AppBarProps) {
  const api = useApi();
  const username = api?.state === "logged-in" ? api.user.username : "UNDEFINED";

  const handleDrawerToggle = () => {
    handleOpenChange(!open);
  };

  const [anchorEl, setAnchorEl] = React.useState<null | HTMLElement>(null);
  const avatarMenuOpen = Boolean(anchorEl);
  const handleAvatarMenuClick = (
    event: React.MouseEvent<HTMLButtonElement>
  ) => {
    setAnchorEl(event.currentTarget);
  };
  const handleAvatarMenuClose = () => {
    setAnchorEl(null);
  };

  return (
    <CustomAppBar position='fixed' elevation={0}>
      <Toolbar
        sx={{
          justifyContent: "space-between",
          backgroundColor: "background.paper",
        }}
      >
        <Box display={{ xs: "flex", md: "none" }}>
          <IconButton onClick={handleDrawerToggle}>
            {open ? <Close /> : <MenuOutlined />}
          </IconButton>
        </Box>
        <Box width='100%' height='100%'>
          <Tooltip title='Archytex version 0.0.1' placement='bottom-start'>
            <Box display={{ xs: "none", md: "flex" }} alignItems='center'>
              <ArchytexIcon />
              <Typography
                variant='h6'
                component='h2'
                fontSize='1em'
                sx={{ display: { xs: "none", sm: "block" } }}
              >
                ARCHYTEX
              </Typography>
            </Box>
          </Tooltip>
        </Box>
        <Box marginX={2} height='100%' display='flex' justifyContent='center'>
          <Typography
            variant='h3'
            fontSize='1.1em'
            component='div'
            sx={{
              display: { xs: "none", sm: "block" },
            }}
          >
            DASHBOARD
          </Typography>
        </Box>
        <Box width='100%' height='100%' display='flex' justifyContent='end'>
          <Button
            variant='text'
            disableRipple
            disableFocusRipple
            disabled
            sx={{
              marginY: 1,
              textTransform: "none",
              display: { xs: "none", md: "block" },
              ":disabled": {
                color: "inherit",
              },
            }}
            color='inherit'
          >
            <Typography
              fontSize='1.2em'
              noWrap
              width={{ md: "280px", lg: "400px", xl: "500px" }}
              textAlign='end'
            >
              {username}
            </Typography>
          </Button>
          <IconButton onClick={handleAvatarMenuClick}>
            <Avatar sx={{ backgroundColor: "#39A0ED" }} />
          </IconButton>
          <Menu
            anchorEl={anchorEl}
            open={avatarMenuOpen}
            onClose={handleAvatarMenuClose}
            PaperProps={{
              elevation: 0,
              sx: {
                overflow: "visible",
                filter: "drop-shadow(0px 2px 8px rgba(0,0,0,0.5))",
                mt: 1.5,
                borderRadius: 2,
                "&:before": {
                  content: '""',
                  display: "block",
                  position: "absolute",
                  top: 0,
                  right: 25,
                  width: 10,
                  height: 10,
                  bgcolor: "paper.background",
                  transform: "translateY(-50%) rotate(45deg)",
                  zIndex: 0,
                },
              },
            }}
            transformOrigin={{ horizontal: "right", vertical: "top" }}
            anchorOrigin={{ horizontal: "right", vertical: "bottom" }}
          >
            <MenuItem>
              <ListItemIcon>
                <AccountCircle fontSize='small' />
              </ListItemIcon>
              Account
            </MenuItem>
            <MenuItem>
              <ListItemIcon>
                <CreditCard fontSize='small' />
              </ListItemIcon>
              Subscription
            </MenuItem>
            <Divider />
            <MenuItem>
              <ListItemIcon>
                <Logout fontSize='small' />
              </ListItemIcon>
              Logout
            </MenuItem>
            {/* Dark mode switch for debugging */}
            <DarkModeSwitch />
          </Menu>
        </Box>
      </Toolbar>
    </CustomAppBar>
  );
}

export default DashboardAppBar;
