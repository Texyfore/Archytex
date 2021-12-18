import { AccountCircle, CreditCard, Logout } from '@mui/icons-material';
import { Button, Typography, IconButton, Menu, MenuItem, ListItemIcon, Avatar, Divider } from '@mui/material';
import React from 'react';
import { useApi } from '../services/user/api';

const UserIconButton = () => {

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
  const api = useApi();
  const username = api?.state === "logged-in" ? api.user.username : "";

  return (
    <>
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
            <Logout fontSize='small'  />
          </ListItemIcon>
          Logout
        </MenuItem>
      </Menu>
    </>
  )
}

export default UserIconButton;