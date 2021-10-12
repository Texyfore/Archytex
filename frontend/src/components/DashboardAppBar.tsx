import React from "react";
import {
  AppBar,
  Avatar,
  Grid,
  IconButton,
  Toolbar,
  Typography,
} from "@mui/material";
import { Box } from "@mui/system";
import ArchytexIcon from "./ArchytexIcon";
import { makeStyles } from "@mui/styles";
import { VerticalAlignCenter } from "@mui/icons-material";

const useStyles = makeStyles(() => {
  return {
    appBarItem: {
      display: "flex",
    },
  };
});

export default function DashboardAppBar() {
  const classes = useStyles();
  return (
    <Box sx={{ flexGrow: 1 }}>
      <AppBar position='static'>
        <Toolbar sx={{ justifyContent: "space-between" }} variant='regular'>
          <Box display={{ xs: "none", md: "flex" }}>
            <ArchytexIcon />
            <Typography variant='h6' component='div'>
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
            <Avatar
              sx={{ backgroundColor: "#39A0ED", marginLeft: 2 }}
              variant='rounded'
            />
          </Box>
        </Toolbar>
      </AppBar>
    </Box>
  );
}
