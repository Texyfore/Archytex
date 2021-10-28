import React from "react";
import {
  Close,
  Logout,
  MenuOutlined,
  PersonAdd,
  Settings,
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
  Divider,
  ListItemIcon,
} from "@mui/material";
import ArchytexIcon from "../ArchytexIcon";
import { styled } from "@mui/material/styles";

const CustomAppBar = styled(AppBar)(({ theme }) => ({
  zIndex: theme.zIndex.drawer + 1,
}));

interface AppBarProps {
  open: boolean;
  handleOpenChange: (value: boolean) => void;
}

function DashboardAppBar({ open, handleOpenChange }: AppBarProps) {
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
    <CustomAppBar position='fixed'>
      <Toolbar sx={{ justifyContent: "space-between" }}>
        <Box display={{ xs: "flex", md: "none" }}>
          <IconButton onClick={handleDrawerToggle}>
            {open ? <Close /> : <MenuOutlined />}
          </IconButton>
        </Box>
        <Tooltip title='Archytex version 0.0.1' placement='right'>
          <Box display={{ xs: "none", md: "flex" }}>
            <ArchytexIcon />
            <Typography
              variant='h6'
              component='h2'
              sx={{ display: { xs: "none", sm: "block" }, paddingTop: 0.5 }}
            >
              ARCHYTEX
            </Typography>
          </Box>
        </Tooltip>
        <Typography
          variant='h6'
          fontSize='.9em'
          component='div'
          sx={{
            display: { xs: "none", sm: "block" },
          }}
        >
          DASHBOARD
        </Typography>
        <Box sx={{ display: "flex" }}>
          <Button
            onClick={handleAvatarMenuClick}
            variant='text'
            disableRipple
            disableFocusRipple
            sx={{
              marginY: 1,
              textTransform: "none",
              display: { xs: "none", md: "block" },
            }}
            color='inherit'
          >
            <Typography fontSize={20} fontWeight={400}>
              Test User
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
                filter: "drop-shadow(0px 2px 8px rgba(0,0,0,0.32))",
                mt: 1.5,
                "&:before": {
                  content: '""',
                  display: "block",
                  position: "absolute",
                  top: 0,
                  right: 25,
                  width: 10,
                  height: 10,
                  bgcolor: "background.paper",
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
                <Logout fontSize='small' />
              </ListItemIcon>
              Logout
            </MenuItem>
          </Menu>
        </Box>
      </Toolbar>
    </CustomAppBar>
  );
}

export default DashboardAppBar;
