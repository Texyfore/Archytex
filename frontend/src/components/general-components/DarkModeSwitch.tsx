import React from "react";

import { useTranslation } from "react-i18next";

import Tooltip from "@mui/material/Tooltip";
import Checkbox from "@mui/material/Checkbox";

import { DarkMode, DarkModeOutlined } from "@mui/icons-material";
import { ColorMode, useColorMode } from "../../services/colorMode";

export default function DarkModeSwitch() {
  const { t } = useTranslation();

  const [colorMode, toggle] = useColorMode();

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
