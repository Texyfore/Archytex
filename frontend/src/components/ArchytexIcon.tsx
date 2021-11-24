import React from "react";
import { Avatar, Theme } from "@mui/material";
import { useTheme } from "@mui/material/styles";
import logo from "../img/logo.svg";
import logoLight from "../img/logoLight.svg";
import { SxProps } from "@mui/system";

interface IconProps {
  size?: Number;
  marginRight?: Number;
}

export default function ArchytexIcon({
  size = 35,
  marginRight = 2,
}: IconProps) {
  const style = {
    height: size,
    width: size,
    marginRight: marginRight,
  } as SxProps<Theme>;
  return (
    <Avatar
      src={useTheme().palette.mode === "dark" ? logo : logoLight}
      alt='ARCHYTEX_LOGO'
      variant='square'
      sx={style}
    />
  );
}
