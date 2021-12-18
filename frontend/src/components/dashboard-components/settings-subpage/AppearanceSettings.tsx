import React from "react";
import { DarkMode, Palette } from "@mui/icons-material";
import {
  Box,
  Divider,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Switch,
  Typography,
} from "@mui/material";
import { ColorMode, useColorMode } from "../../../services/colorMode";

export default function AppearanceSettings() {
  const [colorMode, toggle] = useColorMode();

  return (
    <Box marginBottom={2} marginTop={5}>
      <Box
        marginBottom={2}
        display='flex'
        justifyContent='start'
        paddingLeft={2}
      >
        <Box marginY='auto' marginTop={1.5} marginRight={4}>
          <Palette fontSize='large' />
        </Box>
        <Box>
          <Typography variant='h6'>Appearance</Typography>
          <Typography variant='caption'>Change how Archytex looks</Typography>
        </Box>
      </Box>
      <Divider />
      <List>
        {/* Theme */}
        <ListItem key='theme' disablePadding>
          <ListItemButton onClick={toggle} sx={{ borderRadius: 2 }}>
            <ListItemIcon>
              <DarkMode />
            </ListItemIcon>
            <ListItemText
              primary='Dark mode'
              secondary='Set the application theme to dark.'
            />
            <Switch
              edge='end'
              disabled
              onChange={toggle}
              checked={colorMode === ColorMode.Dark}
            />
          </ListItemButton>
        </ListItem>
      </List>
    </Box>
  );
}
