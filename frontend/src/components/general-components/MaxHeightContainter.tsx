import React from "react";

import { styled } from "@mui/material/styles";
import Box from "@mui/material/Box";

const MaxHeightContainer = styled(Box)(({ theme }) => ({
  marginTop: 56,
  height: `calc(100vh - 56px)`,
  [`${theme.breakpoints.up("xs")} and (orientation: landscape)`]: {
    marginTop: 48,
    height: `calc(100vh - 48px)`,
  },
  [theme.breakpoints.up("sm")]: {
    marginTop: 64,
    height: `calc(100vh - 64px)`,
  },
  // eslint-disable-next-line no-useless-computed-key
  ["@media (max-height: 700px)"]: {
    height: "unset",
  },
}));

export default MaxHeightContainer;
