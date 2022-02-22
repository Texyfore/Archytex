import React from "react";

import { Theme } from "@mui/material";
import { useTheme } from "@mui/material/styles";
import { SxProps } from "@mui/system";
import Avatar from "@mui/material/Avatar";

import logoLight from "../../img/logoLight.svg";
import logoDark from "../../img/logoDark.svg";

interface LogoProps {
  size?: number;
  marginRight?: number;
}

export default function Logo({ size = 35, marginRight = 2 }: LogoProps) {
  const style = {
    height: size,
    width: size,
    marginRight: marginRight,
  } as SxProps<Theme>;

  return (
    <Avatar
      src={useTheme().palette.mode === "dark" ? logoDark : logoLight}
      alt='ARCHYTEX_LOGO'
      variant='square'
      sx={style}
    />
  );
}
