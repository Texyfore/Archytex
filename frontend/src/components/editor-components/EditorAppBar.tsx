import React, { useState } from "react";

import { useTranslation } from "react-i18next";

import { styled } from "@mui/material/styles";

import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Box from "@mui/material/Box";
import Tooltip from "@mui/material/Tooltip";
import Button from "@mui/material/Button";
import IconButton from "@mui/material/IconButton";
import Menu from "@mui/material/Menu";

import { MoreVert } from "@mui/icons-material";

import DarkModeSwitch from "../general-components/DarkModeSwitch";
import LanguageSelectDropdown from "../general-components/LanguageSelectDropdown";

const CustomEditorAppBar = styled(AppBar)(({ theme }) => ({
  zIndex: theme.zIndex.drawer + 1,
  backgroundColor: theme.palette.background.paper,
}));

interface EditorAppBarProps {
  onSave: (type: "export" | "save" | "render") => void;
}

export default function EditorAppBar({ onSave }: EditorAppBarProps) {
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
  const { t } = useTranslation();
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
  const tooltipText: string =
    t("archytex") + " " + t("version") + " " + "1.0.0";
  return (
    <>
      <CustomEditorAppBar elevation={0}>
        <Toolbar variant='dense' sx={{ borderBottom: "1px solid #1F1F1F" }}>
          <Box
            display='flex'
            justifyContent='space-between'
            width='100%'
            alignItems='center'
          >
            <Box width='100%' height='100%' display='flex' alignItems='center'>
              <Box height='100%' display='flex' alignItems='center'>
                <Tooltip title={tooltipText} placement='bottom-start'>
                  <Box>{/* <ArchytexIcon size={25} /> */}</Box>
                </Tooltip>
              </Box>

              {/* Appbar menu */}
              <Box display='flex'>
                <Button
                  variant='text'
                  color='inherit'
                  sx={{ textTransform: "none" }}
                  onClick={() => onSave("save")}
                >
                  {t("save")}
                </Button>
                <Button
                  variant='text'
                  color='inherit'
                  sx={{ textTransform: "none" }}
                  onClick={() => onSave("export")}
                >
                  {t("export")}
                </Button>
                <Button
                  variant='text'
                  color='inherit'
                  sx={{ textTransform: "none" }}
                  onClick={() => onSave("render")}
                >
                  {t("render")}
                </Button>
              </Box>
            </Box>
            <IconButton
              size='small'
              onClick={handleOptionsClick}
              sx={{
                display: { xs: "none", md: "initial" },
                width: "34px",
                height: "34px",
              }}
            >
              <MoreVert />
            </IconButton>
          </Box>
        </Toolbar>
      </CustomEditorAppBar>

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
        sx={{ marginTop: 1 }}
      >
        <Box
          width={100}
          paddingY={0}
          display='flex'
          justifyContent='space-evenly'
        >
          <DarkModeSwitch />
          <LanguageSelectDropdown />
        </Box>
      </Menu>
    </>
  );
}
