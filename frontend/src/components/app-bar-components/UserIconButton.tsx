import React from "react";

import IconButton from "@mui/material/IconButton";
import Avatar from "@mui/material/Avatar";

import { useApi } from "../../services/user/api";

import UserDropdownMenu from "./UserDropdownMenu";

export default function UserIconButton() {
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
    <>
      <IconButton onClick={handleAvatarMenuClick}>
        <Avatar sx={{ width: 32, height: 32 }} />
      </IconButton>
      <UserDropdownMenu
        anchorEl={anchorEl}
        avatarMenuOpen={avatarMenuOpen}
        handleAvatarMenuClose={handleAvatarMenuClose}
      />
    </>
  );
}
