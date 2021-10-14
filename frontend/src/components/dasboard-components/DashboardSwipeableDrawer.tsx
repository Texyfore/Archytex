import React from "react";
import {
  Avatar,
  Box,
  Divider,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  SwipeableDrawer,
  Typography,
} from "@mui/material";
import { styled } from "@mui/material/styles";
import { BoltOutlined, Inbox, MailOutlined } from "@mui/icons-material";
import { blue } from "@mui/material/colors";
import DashboardUserData from "./DashboardUserData";

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

export default function DashboardSwipeableDrawer({
  open,
  handleOpenChange,
}: SwipeableDrawerProps) {
  return (
    <SwipeableDrawer
      anchor='left'
      open={open}
      onClose={() => handleOpenChange(false)}
      onOpen={() => handleOpenChange(true)}
      sx={{ display: { xs: "flex", md: "none" } }}
    >
      <DrawerHeader sx={{ width: 300 }} />
      <DashboardUserData />
      <List>
        {["Inbox", "Starred", "Send email", "Drafts"].map((text, index) => (
          <ListItem button key={text}>
            <ListItemIcon>
              {index % 2 === 0 ? <Inbox /> : <MailOutlined />}
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
              {index % 2 === 0 ? <Inbox /> : <MailOutlined />}
            </ListItemIcon>
            <ListItemText primary={text} />
          </ListItem>
        ))}
      </List>
    </SwipeableDrawer>
  );
}
