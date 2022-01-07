import React from "react";
import { Checkbox, Tooltip } from "@mui/material";
import { DarkMode, DarkModeOutlined } from "@mui/icons-material";
import { ColorMode, useColorMode } from "../services/colorMode";
import { useTranslation } from "react-i18next";

export default function DarkModeSwitch() {
  const [colorMode, toggle] = useColorMode();
  const { t } = useTranslation();
  const handleDarkModeChange = () => {
    toggle();
  };
  const tooltipText: string = t("toggle_dark_mode");
  return (
    <Tooltip title={tooltipText}>
      <Checkbox
        icon={<DarkModeOutlined />}
        checkedIcon={<DarkMode />}
        checked={colorMode === ColorMode.Dark}
        onChange={handleDarkModeChange}
      />
    </Tooltip>
  );
}
