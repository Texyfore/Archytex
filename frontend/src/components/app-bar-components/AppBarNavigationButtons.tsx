import React from "react";

import Stack from "@mui/material/Stack";
import Button from "@mui/material/Button";
import Divider from "@mui/material/Divider";

export default function AppBarNavigationButtons() {
  return (
    <Stack
      direction='row'
      spacing={2}
      divider={<Divider orientation='vertical' flexItem />}
      pl={4}
      display={{ xs: "none", md: "inherit" }}
    >
      <Button variant='text'>Home</Button>
      <Button variant='text'>Community</Button>
      <Button variant='text'>Dashboard</Button>
    </Stack>
  );
}
