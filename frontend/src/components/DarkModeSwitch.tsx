import React, { useContext } from "react";
import { Checkbox, Tooltip } from "@mui/material";
import { DarkMode, DarkModeOutlined } from "@mui/icons-material";
import { ColorModeContext } from "../App";
import { useTheme } from "@mui/material/styles";

export default function DarkModeSwitch() {
  const colorMode = useContext(ColorModeContext);
  const handleDarkModeChange = () => {
    colorMode.toggleColorMode();
  };
  return (
    <Tooltip title='Toggle dark mode'>
      <Checkbox
        icon={<DarkModeOutlined />}
        checkedIcon={<DarkMode />}
        checked={useTheme().palette.mode === "dark"}
        onChange={handleDarkModeChange}
      />
    </Tooltip>
  );
}
