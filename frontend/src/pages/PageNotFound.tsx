import { MoodBadRounded } from "@mui/icons-material";
import { Box, Link, Typography } from "@mui/material";
import React, { useState } from "react";
import ArchytexAppBar from "../components/ArchytexAppBar";
import ArchytexFooter from "../components/ArchytexFooter";
import MaxHeightContainer from "../components/MaxHeightContainer";

export default function PageNotFound() {
  const [open, setOpen] = useState(false);
  function handleOpenChange(value: boolean): void {
    setOpen(value);
  }
  return (
    <React.Fragment>
      <ArchytexAppBar open={open} handleOpenChange={handleOpenChange} />
      <MaxHeightContainer
        display={{ md: "flex" }}
        justifyContent='center'
        alignItems='center'
      >
        <Box
          marginRight={{ md: 5, lg: 10 }}
          marginBottom={{ xs: 5, md: 0 }}
          marginTop={{ xs: 10, md: 0 }}
          display='flex'
          justifyContent='center'
        >
          <MoodBadRounded sx={{ width: 200, height: 200 }} color='error' />
        </Box>
        <Box paddingX={{ xs: 2, md: 0 }}>
          <Typography
            gutterBottom
            variant='h2'
            textAlign={{ xs: "center", md: "left" }}
          >
            404
          </Typography>
          <Typography variant='subtitle1' textAlign='justify'>
            The page you're looking for does not exist.
          </Typography>
          <Typography variant='subtitle1' textAlign='justify'>
            <Link href='/'>Back to main page</Link>
          </Typography>
        </Box>
      </MaxHeightContainer>
      <ArchytexFooter />
    </React.Fragment>
  );
}
