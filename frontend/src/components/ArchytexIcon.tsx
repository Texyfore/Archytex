import { Avatar } from "@mui/material";
import React from "react";
import logo from "../logo.svg";

export default function ArchytexIcon() {
  return (
    <Avatar
      src={logo}
      alt='ARCHYTEX_LOGO'
      variant='square'
      sx={{ marginRight: 2 }}
    />
  );
}
