import React from "react";

import Box from "@mui/material/Box";

import DarkModeSwitch from "../general-components/DarkModeSwitch";
import LanguageSelectDropdown from "../general-components/LanguageSelectDropdown";

export default function DrawerBottomButtons() {
  return (
    <Box
      marginTop='auto'
      marginBottom={2}
      display='flex'
      alignItems='end'
      justifyContent='space-evenly'
    >
      <DarkModeSwitch />
      <LanguageSelectDropdown />
    </Box>
  );
}
