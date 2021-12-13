import React, { useState } from "react";
import { Box, Link, Typography } from "@mui/material";
import { MoodRounded } from "@mui/icons-material";
import ArchytexAppBar from "../components/ArchytexAppBar";
import ArchytexFooter from "../components/ArchytexFooter";
import MaxHeightContainer from "../components/MaxHeightContainer";

export default function SuccessfulRegistration() {
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
          <MoodRounded sx={{ width: 200, height: 200 }} color='primary' />
        </Box>
        <Box paddingX={{ xs: 2, md: 0 }}>
          <Typography
            gutterBottom
            variant='h2'
            textAlign={{ xs: "center", md: "left" }}
          >
            Registration successful!
          </Typography>
          <Typography variant='subtitle1' textAlign='justify'>
            Please verify your account by checking the email we sent you. After
            doing so, you will be able to log in <Link href='/login'>here</Link>
            .
          </Typography>
        </Box>
      </MaxHeightContainer>
      <ArchytexFooter />
    </React.Fragment>
  );
}
