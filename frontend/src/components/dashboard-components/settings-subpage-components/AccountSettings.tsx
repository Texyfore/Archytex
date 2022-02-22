import React from "react";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import Button from "@mui/material/Button";
import Divider from "@mui/material/Divider";
import List from "@mui/material/List";
import ListItem from "@mui/material/ListItem";
import ListItemIcon from "@mui/material/ListItemIcon";
import ListItemText from "@mui/material/ListItemText";
import TextField from "@mui/material/TextField";
import Typography from "@mui/material/Typography";

import {
  AccountBox,
  Email,
  ManageAccounts,
  Password,
} from "@mui/icons-material";

export default function AccountSettings() {
  const { t } = useTranslation();

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
          <Typography variant='h6'>{t("account")}</Typography>
          <Typography variant='caption'>
            {t("your_personal_settings")}
          </Typography>
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
              primary={t("username")}
              secondary={t("your_name_that_everyone_will_see")}
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
              {t("update")}
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
              primary={t("email")}
              secondary={t("the_email_we_use_to_contact_you")}
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
              {t("update")}
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
              primary={t("password")}
              secondary={t("change_your_password")}
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
              {t("update")}
            </Button>
          </Box>
        </Box>
      </List>
    </Box>
  );
}
