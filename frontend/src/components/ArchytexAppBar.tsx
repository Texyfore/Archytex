import React from "react";
import { Close, MenuOutlined } from "@mui/icons-material";
import {
  AppBar,
  IconButton,
  Toolbar,
  Typography,
  Box,
  Tooltip,
  Button,
} from "@mui/material";
import ArchytexIcon from "./ArchytexIcon";
import { styled } from "@mui/material/styles";
import MainPageSwipeableDrawer from "./main-page-components/MainPageSwipeableDrawer";
import LanguageSelectDropdown from "./LanguageSelectDropdown";
import DarkModeSwitch from "./DarkModeSwitch";

const CustomAppBar = styled(AppBar)(({ theme }) => ({
  zIndex: theme.zIndex.drawer + 1,
  filter: "drop-shadow(0px 2px 4px rgba(0,0,0,0.5))",
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
  const languageMenuOpen = Boolean(anchorEl);
  const handleLanguageMenuClick = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };
  const handleLanguageMenuClose = () => {
    setAnchorEl(null);
  };
  return (
    <React.Fragment>
      <CustomAppBar position='fixed' elevation={0}>
        <Toolbar sx={{ justifyContent: "space-between" }}>
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
          <Box
            marginX={2}
            height='100%'
            display={{ xs: "none", md: "flex" }}
            justifyContent='space-between'
            gap={2}
          >
            <Button color='inherit' variant='text'>
              Home
            </Button>
            <Button color='inherit' variant='text'>
              Community
            </Button>
          </Box>
          <Box
            width='100%'
            height='100%'
            display={{ xs: "none", md: "flex" }}
            justifyContent='end'
          >
            <DarkModeSwitch />
            <LanguageSelectDropdown
              open={languageMenuOpen}
              handleClick={handleLanguageMenuClick}
              handleClose={handleLanguageMenuClose}
              anchorEl={anchorEl}
            />
            <Button variant='outlined' sx={{ marginLeft: 2 }}>
              Login / register
            </Button>
          </Box>
        </Toolbar>
      </CustomAppBar>
      <MainPageSwipeableDrawer
        open={open}
        handleOpenChange={handleOpenChange}
      />
    </React.Fragment>
  );
}

export default DashboardAppBar;
