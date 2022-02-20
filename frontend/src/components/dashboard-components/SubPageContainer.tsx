import React from "react";

import Box from "@mui/material/Box";
import Grow from "@mui/material/Grow";

import { useSubPage } from "../../services/selectedDashboardSubPage";

interface Props {
  trigger: string;
  children: null | JSX.Element | JSX.Element[];
}
export default function SubPageContainer({ trigger, children }: Props) {
  const [subpage, _] = useSubPage();

  return (
    <Grow in={subpage === trigger}>
      <Box height='100%' display={subpage === trigger ? "block" : "none"}>
        {children}
      </Box>
    </Grow>
  );
}
