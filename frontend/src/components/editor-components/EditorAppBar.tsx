import React, { useState } from "react";

import { useTranslation } from "react-i18next";

import { styled } from "@mui/material/styles";

import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Box from "@mui/material/Box";
import Tooltip from "@mui/material/Tooltip";
import Button from "@mui/material/Button";

import Logo from "../general-components/Logo";
import RenderSetupModal from "./RenderSetupModal";
import LanguageSelectDropdown from "../general-components/LanguageSelectDropdown";
import DarkModeSwitch from "../general-components/DarkModeSwitch";

import { ColorMode, useColorMode } from "../../services/colorMode";

const CustomEditorAppBar = styled(AppBar)(({ theme }) => ({
  zIndex: theme.zIndex.drawer + 1,
  backgroundColor: theme.palette.background.paper,
}));

interface EditorAppBarProps {
  onSave:  (type: "export" | "save") => Promise<void>;
  onRender: (width: number, height: number, samples: number) => Promise<void>;
}

export default function EditorAppBar({ onSave, onRender }: EditorAppBarProps) {
  const { t } = useTranslation();
  const tooltipText: string =
    t("archytex") + " " + t("version") + " " + t("version_number");

  const [colorMode, _] = useColorMode();

  const [renderSetupModalOpen, setRenderSetupModalOpen] = useState(false);
  const handleRenderSetupModalOpen = () => setRenderSetupModalOpen(true);
  const handleRenderSetupModalClose = () => setRenderSetupModalOpen(false);

  return (
    <>
      <CustomEditorAppBar elevation={0}>
        <Toolbar
          variant='dense'
          sx={{
            borderBottom:
              colorMode === ColorMode.Dark
                ? "1px solid #2E2E2E"
                : "1px solid #BABABA",
          }}
        >
          <Box
            display='flex'
            justifyContent='space-between'
            width='100%'
            alignItems='center'
          >
            <Box width='100%' height='100%' display='flex' alignItems='center'>
              <Box height='100%' display='flex' alignItems='center'>
                <Tooltip title={tooltipText} placement='bottom-start'>
                  <Box>
                    <Logo />
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
                  onClick={handleRenderSetupModalOpen}
                >
                  {t("render")}
                </Button>
              </Box>
            </Box>
            <Box alignSelf='right' display='flex' flexWrap='nowrap'>
              <LanguageSelectDropdown />
              <DarkModeSwitch />
            </Box>
          </Box>
        </Toolbar>
      </CustomEditorAppBar>

      <RenderSetupModal
        modalOpen={renderSetupModalOpen}
        handleModalOpen={handleRenderSetupModalOpen}
        handleModalClose={handleRenderSetupModalClose}
        onRender={onRender}
      />
    </>
  );
}
