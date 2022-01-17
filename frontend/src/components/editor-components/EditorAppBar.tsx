import React, { useState } from "react";
import {
  AppBar,
  Box,
  Button,
  IconButton,
  Menu,
  Toolbar,
  Tooltip,
} from "@mui/material";
import { styled } from "@mui/material/styles";
import ArchytexIcon from "../ArchytexIcon";
import DarkModeSwitch from "../DarkModeSwitch";
import LanguageSelectDropdown from "../LanguageSelectDropdown";
import { MoreVert } from "@mui/icons-material";
import { useTranslation } from "react-i18next";

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
                <Tooltip
                  title='Archytex version 0.0.1'
                  placement='bottom-start'
                >
                  <Box>
                    <ArchytexIcon size={25} />
                  </Box>
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
                  Save
                </Button>
                <Button
                  variant='text'
                  color='inherit'
                  sx={{ textTransform: "none" }}
                  onClick={() => onSave("export")}
                >
                  Export
                </Button>
                <Button
                  variant='text'
                  color='inherit'
                  sx={{ textTransform: "none" }}
                  onClick={() => onSave("render")}
                >
                  Render
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
