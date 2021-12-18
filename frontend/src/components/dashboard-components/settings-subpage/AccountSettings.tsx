import React from "react";
import {
  AccountBox,
  Email,
  ManageAccounts,
  Password,
} from "@mui/icons-material";
import {
  Box,
  Button,
  Divider,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  TextField,
  Typography,
} from "@mui/material";

export default function AccountSettings() {
  return (
    <Box marginBottom={2} marginTop={5}>
      <Box
        marginBottom={2}
        display='flex'
        justifyContent='start'
        paddingLeft={2}
      >
        <Box marginY='auto' marginTop={1.5} marginRight={4}>
          <ManageAccounts fontSize='large' />
        </Box>
        <Box>
          <Typography variant='h6'>Account</Typography>
          <Typography variant='caption'>Your personal settings</Typography>
        </Box>
      </Box>
      <Divider />
      <List>
        {/* Username */}
        <Box display='flex' justifyContent='start' flexWrap='wrap'>
          <ListItem
            disablePadding
            sx={{ paddingX: 2, width: { xs: "100%", md: "50%" } }}
          >
            <ListItemIcon>
              <AccountBox />
            </ListItemIcon>
            <ListItemText
              primary='Username'
              secondary='Your name, that everyone will see.'
            />
          </ListItem>
          <Box
            marginBottom={2}
            paddingX={2}
            display='flex'
            justifyContent='start'
          >
            <TextField
              size='small'
              id='username'
              variant='outlined'
              margin='none'
            />
            <Button disableElevation variant='contained'>
              Update
            </Button>
          </Box>
        </Box>
        {/* Email */}
        <Box display='flex' justifyContent='start' flexWrap='wrap'>
          <ListItem
            disablePadding
            sx={{ paddingX: 2, width: { xs: "100%", md: "50%" } }}
          >
            <ListItemIcon>
              <Email />
            </ListItemIcon>
            <ListItemText
              primary='Email'
              secondary='The email we use to contact you.'
            />
          </ListItem>
          <Box
            marginBottom={2}
            paddingX={2}
            display='flex'
            justifyContent='start'
          >
            <TextField
              size='small'
              id='email'
              variant='outlined'
              margin='none'
            />
            <Button disableElevation variant='contained'>
              Update
            </Button>
          </Box>
        </Box>
        {/* Password */}
        <Box display='flex' justifyContent='start' flexWrap='wrap'>
          <ListItem
            disablePadding
            sx={{ paddingX: 2, width: { xs: "100%", md: "50%" } }}
          >
            <ListItemIcon>
              <Password />
            </ListItemIcon>
            <ListItemText
              primary='Password'
              secondary='Change your password.'
            />
          </ListItem>
          <Box
            marginBottom={2}
            paddingX={2}
            display='flex'
            justifyContent='start'
          >
            <TextField
              size='small'
              id='password'
              variant='outlined'
              margin='none'
            />
            <Button disableElevation variant='contained'>
              Update
            </Button>
          </Box>
        </Box>
      </List>
    </Box>
  );
}
