import React, { useState } from "react";
import { Close, MenuOutlined, MoreVert } from "@mui/icons-material";
import { AppBar, IconButton, Toolbar, Box, Button, Menu } from "@mui/material";
import { styled } from "@mui/material/styles";
import ArchytexLogoWithText from "./ArchytexLogoWithText";
import UserIconButton from "./UserIconButton";
import GeneralSwipeableDrawer from "./GeneralSwipeableDrawer";
import { useHistory } from "react-router-dom";
import { useApi } from "../services/user/api";
import { useTranslation } from "react-i18next";
import DarkModeSwitch from "./DarkModeSwitch";
import LanguageSelectDropdown from "./LanguageSelectDropdown";
import { height } from "@mui/system";

const CustomAppBar = styled(AppBar)(({ theme }) => ({
  zIndex: theme.zIndex.drawer + 1,
  filter: "drop-shadow(0px 2px 4px rgba(0,0,0,0.5))",
}));

interface AppBarProps {
  content: "general" | "dashboard";
}

function ArchytexAppBar({ content }: AppBarProps) {
  const api = useApi();
  const history = useHistory();

  const { t } = useTranslation();

  const [open, setOpen] = useState(false);
  const handleOpenChange = (value: boolean) => {
    setOpen(value);
  };
  const handleDrawerToggle = () => {
    handleOpenChange(!open);
  };

  //Options menu
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);
  const optionsOpen = Boolean(anchorEl);
  const handleOptionsClick = (event: React.MouseEvent<HTMLButtonElement>) => {
    setAnchorEl(event.currentTarget);
  };
  const handleOptionsClose = () => {
    setAnchorEl(null);
  };

  //Language select dropdown
  const [languageAnchorEl, setLanguageAnchorEl] = useState<null | HTMLElement>(
    null
  );
  const languageOpen = Boolean(languageAnchorEl);
  const handleLanguageClick = (
    event: React.MouseEvent<HTMLElement, MouseEvent>
  ) => {
    setLanguageAnchorEl(event.currentTarget);
  };
  const handleLanguageClose = () => {
    setLanguageAnchorEl(null);
  };

  return (
    <>
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
            <ArchytexLogoWithText />
          </Box>
          <Box
            marginX={2}
            height='100%'
            display={{ xs: "none", md: "flex" }}
            justifyContent='space-between'
            gap={2}
          >
            <Button
              color='inherit'
              variant='text'
              onClick={() => history.push("/")}
            >
              {t("home")}
            </Button>
            {api?.state === "logged-in" ? (
              <Button
                color='inherit'
                variant='text'
                onClick={() => history.push("/dashboard")}
              >
                {t("dashboard")}
              </Button>
            ) : null}
          </Box>
          <Box width='100%' height='100%' display='flex' justifyContent='end'>
            {api?.state === "not-logged-in" ? (
              <Button variant='outlined' onClick={() => history.push("/login")}>
                Login
              </Button>
            ) : (
              <UserIconButton />
            )}
          </Box>
          <IconButton
            onClick={handleOptionsClick}
            sx={{
              display: { xs: "none", md: "initial" },
              width: "40px",
              height: "40px",
              marginLeft: 2,
            }}
          >
            <MoreVert />
          </IconButton>
        </Toolbar>
      </CustomAppBar>

      <GeneralSwipeableDrawer
        content={content}
        open={open}
        handleOpenChange={handleOpenChange}
      />

      <Menu
        anchorEl={anchorEl}
        open={optionsOpen}
        onClose={handleOptionsClose}
        anchorOrigin={{
          vertical: "bottom",
          horizontal: "right",
        }}
        transformOrigin={{
          vertical: "top",
          horizontal: "right",
        }}
        sx={{ marginTop: 2 }}
      >
        <Box
          width={100}
          paddingY={0}
          display='flex'
          justifyContent='space-evenly'
        >
          <DarkModeSwitch />
          <LanguageSelectDropdown
            open={languageOpen}
            anchorEl={languageAnchorEl}
            handleClick={handleLanguageClick}
            handleClose={handleLanguageClose}
          />
        </Box>
      </Menu>
    </>
  );
}

export default ArchytexAppBar;
