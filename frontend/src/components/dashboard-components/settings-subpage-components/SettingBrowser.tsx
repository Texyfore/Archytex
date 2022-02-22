import React from "react";

import { styled } from "@mui/material/styles";
import { Box } from "@mui/material";

import AccountSettings from "./AccountSettings";
import AppearanceSettings from "./AppearanceSettings";

const SettingContainer = styled(Box)(({ theme }) => ({
  height: "calc(100vh - 65px)",
  overflowY: "scroll",

  [`${theme.breakpoints.up("xs")} and (orientation: landscape)`]: {
    height: "calc(100vh - 49px - 48px)",
  },
  [theme.breakpoints.up("sm")]: {
    height: "calc(100vh - 65px - 48px)",
  },
  [theme.breakpoints.up("md")]: {
    height: "calc(100vh - 65px)",
  },
}));

export default function SettingsBrowser() {
  return (
    <SettingContainer>
      <AccountSettings />
      <AppearanceSettings />
    </SettingContainer>
  );
}
