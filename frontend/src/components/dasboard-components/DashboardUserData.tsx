import React from "react";
import { BoltOutlined } from "@mui/icons-material";
import { Avatar, Box, Typography } from "@mui/material";
import { styled } from "@mui/material/styles";
import { blue } from "@mui/material/colors";

const ContentBox = styled(Box)(({ theme }) => ({
  display: "flex",
  flexDirection: "column",
  justifyContent: "center",
}));

const UserAvatar = styled(Avatar)(({ theme }) => ({
  bgcolor: blue[500],
  color: "white",
  width: "2.5em",
  height: "2.5em",
  alignSelf: "center",
  fontSize: "30pt",
}));
export default function DashboardUserData() {
  return (
    //TODO: Collapse animation
    <ContentBox
      sx={{
        paddingTop: { xs: 5, md: 0, lg: 5, xl: 6 },
        marginBottom: { md: 4, lg: 0 },
        gap: { lg: 2 },
      }}
    >
      <UserAvatar />
      <Typography
        variant='caption'
        fontSize='16pt'
        fontWeight={600}
        textAlign='center'
      >
        Test User
      </Typography>
      <Box display='flex' justifyContent='center'>
        <BoltOutlined />
        <Typography noWrap>1003</Typography>
      </Box>
    </ContentBox>
  );
}
