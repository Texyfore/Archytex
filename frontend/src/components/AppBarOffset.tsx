import React from "react";
import { styled } from "@mui/material/styles";

const Offset = styled("div")(({ theme }) => ({
  height: `56px`,
  [`${theme.breakpoints.up("xs")} and (orientation: landscape)`]: {
    height: `48px`,
  },
  [theme.breakpoints.up("sm")]: {
    height: `64px`,
  },
}));

const DenseOffset = styled("div")(({ theme }) => ({
  height: `48px`,
}));

type variant = "regular" | "dense";
interface AppBarOffsetProps {
  variant?: variant;
}
export default function AppBarOffset({
  variant = "regular",
}: AppBarOffsetProps): JSX.Element {
  switch (variant) {
    case "regular":
      return <Offset />;
    case "dense":
      return <DenseOffset />;
    default:
      return <Offset />;
  }
}
