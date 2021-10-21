import { LinearScale, Logout } from "@mui/icons-material";
import { Button } from "@mui/material";
import { Box } from "@mui/system";
import React from "react";

export default function DashboardLogOut() {
  return (
    <Box sx={{ display: "flex", justifyContent: "space-between", margin: 2 }}>
      <Button size='small' color='inherit' startIcon={<LinearScale />}>
        v0.0.1
      </Button>
      <Button size='small' color='inherit' endIcon={<Logout />}>
        Log out
      </Button>
    </Box>
  );
}
