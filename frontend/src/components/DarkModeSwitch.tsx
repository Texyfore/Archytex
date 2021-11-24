import React, { useContext } from "react";
import { Checkbox, Tooltip } from "@mui/material";
import { DarkMode, DarkModeOutlined } from "@mui/icons-material";
import { useTheme } from "@mui/material/styles";
import { ColorMode, useColorMode } from "../services/colorMode";

export default function DarkModeSwitch() {
  const [colorMode, toggle] = useColorMode();
  const handleDarkModeChange = () => {
    toggle();
  };
  return (
    <Tooltip title='Toggle dark mode'>
      <Checkbox
        icon={<DarkModeOutlined />}
        checkedIcon={<DarkMode />}
        checked={colorMode === ColorMode.Dark}
        onChange={handleDarkModeChange}
      />
    </Tooltip>
  );
}
