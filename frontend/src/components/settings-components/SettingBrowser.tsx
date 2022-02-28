import React from "react";

import { styled } from "@mui/material/styles";
import { Box } from "@mui/material";

import AccountSettings from "./AccountSettings";
import AppearanceSettings from "./AppearanceSettings";
import AppBarOffset from "../app-bar-components/AppBarOffset";

export default function SettingsBrowser() {
  return (
    <>
      <AccountSettings />
      <AppearanceSettings />
    </>
  );
}
