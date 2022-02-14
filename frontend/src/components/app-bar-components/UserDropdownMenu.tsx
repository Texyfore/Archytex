import React from "react";

import { useHistory } from "react-router-dom";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import Avatar from "@mui/material/Avatar";
import Menu from "@mui/material/Menu";
import MenuItem from "@mui/material/MenuItem";
import ListItemIcon from "@mui/material/ListItemIcon";
import Divider from "@mui/material/Divider";

import { AccountCircle, CreditCard, Logout } from "@mui/icons-material";

import { useApi } from "../../services/user/api";

interface Props {
  anchorEl: null | HTMLElement;
  avatarMenuOpen: boolean;
  handleAvatarMenuClose: () => void;
}

export default function UserDropdownMenu({
  anchorEl,
  avatarMenuOpen,
  handleAvatarMenuClose,
}: Props) {
  const { t } = useTranslation();

  const history = useHistory();

  const api = useApi();

  const handleLogOut = () => {
    if (api?.state === "logged-in") {
      history.push("/");
      api.logOut();
    }
  };

  return (
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
      <Box
        display='flex'
        flexDirection='column'
        justifyContent='center'
        alignItems='center'
        my={2}
      >
        <Avatar sx={{ marginBottom: 1, width: 50, height: 50 }} />
        <Typography variant='caption'>
          {api?.state === "logged-in" ? api.user.username : ""}
        </Typography>
      </Box>
      <Divider />
      <MenuItem>
        <ListItemIcon>
          <AccountCircle fontSize='small' />
        </ListItemIcon>
        {t("account")}
      </MenuItem>
      <MenuItem>
        <ListItemIcon>
          <CreditCard fontSize='small' />
        </ListItemIcon>
        {t("subscription")}
      </MenuItem>
      <Divider />
      <MenuItem onClick={handleLogOut}>
        <ListItemIcon>
          <Logout fontSize='small' />
        </ListItemIcon>
        {t("log_out")}
      </MenuItem>
    </Menu>
  );
}
