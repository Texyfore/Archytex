import React from "react";
import { Box } from "@mui/material";
import { styled } from "@mui/material/styles";
import AccountSettings from "./AccountSettings";
import AppearanceSettings from "./AppearanceSettings";
import SubscriptionSettings from "./SubscriptionSettings";

const headerHeight = 50;
const SettingContainer = styled(Box)(({ theme }) => ({
  border: "none",
  overflowY: "scroll",
  height: `calc(100vh - 56px - ${headerHeight}px)`,
  [`${theme.breakpoints.up("xs")} and (orientation: landscape)`]: {
    height: `calc(100vh - 48px - ${headerHeight}px)`,
  },
  [theme.breakpoints.up("sm")]: {
    height: `calc(100vh - 64px - ${headerHeight}px)`,
  },
  [theme.breakpoints.up("lg")]: {
    height: `calc(100% - ${headerHeight}px)`,
  },
}));

export default function SettingsBrowser() {
  //TODO: Use scrollspy
  // https://codesandbox.io/s/material-demo-xu80m?file=/ScrollSpyTabs.js
  return (
    <SettingContainer>
      <AccountSettings />
      <AppearanceSettings />
      <SubscriptionSettings />
    </SettingContainer>
  );
}
