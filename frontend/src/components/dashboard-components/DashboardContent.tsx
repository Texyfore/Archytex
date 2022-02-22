import React from "react";

import { useTheme } from "@mui/material/styles";
import useMediaQuery from "@mui/material/useMediaQuery";

import DesktopDashboard from "./DesktopDashboard";
import MobileDashboard from "./MobileDashboard";

export default function DashboardRightContent() {
  const theme = useTheme();
  const small = useMediaQuery(theme.breakpoints.down("md"));

  return small ? <MobileDashboard /> : <DesktopDashboard />;
}
