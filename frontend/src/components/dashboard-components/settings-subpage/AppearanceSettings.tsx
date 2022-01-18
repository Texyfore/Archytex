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
import { useTranslation } from "react-i18next";

export default function AppearanceSettings() {
  const { t } = useTranslation();

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
          <Typography variant='h6'>{t("appearance")}</Typography>
          <Typography variant='caption'>
            {t("change_how_archytex_looks")}
          </Typography>
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
              primary={t("dark_mode")}
              secondary={t("set_the_application_theme_to_dark")}
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
