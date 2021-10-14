import React from "react";
import { BoltOutlined } from "@mui/icons-material";
import { Avatar, Box, Typography } from "@mui/material";
import { styled } from "@mui/material/styles";
import { blue } from "@mui/material/colors";

const ContentBox = styled(Box)(({ theme }) => ({
  display: "flex",
  flexDirection: "column",
  justifyContent: "center",
  gap: theme.spacing(2),
  padding: theme.spacing(4),
}));

export default function DashboardUserData() {
  return (
    //TODO: Collapse animation
    <ContentBox>
      <Avatar
        sx={{
          bgcolor: blue[500],
          color: "white",
          width: "2em",
          height: "2em",
          alignSelf: "center",
          fontSize: "30pt",
        }}
      />
      <Typography variant='h5' fontWeight={600} textAlign='center'>
        Test User
      </Typography>
      <Box display='flex' justifyContent='center'>
        <BoltOutlined />
        <Typography noWrap>1003</Typography>
      </Box>
    </ContentBox>
  );
}
