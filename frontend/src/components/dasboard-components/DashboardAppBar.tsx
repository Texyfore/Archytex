import React from "react";
import { Close, MenuOutlined } from "@mui/icons-material";
import {
  AppBar,
  Avatar,
  IconButton,
  Toolbar,
  Typography,
  Box,
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

  return (
    <CustomAppBar position='fixed'>
      <Toolbar sx={{ justifyContent: "space-between" }}>
        <Box display={{ xs: "flex", md: "none" }}>
          <IconButton onClick={handleDrawerToggle}>
            {open ? <Close /> : <MenuOutlined />}
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
    </CustomAppBar>
  );
}

export default DashboardAppBar;
