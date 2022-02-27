import React from "react";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import Divider from "@mui/material/Divider";
import List from "@mui/material/List";
import ListItem from "@mui/material/ListItem";
import ListItemButton from "@mui/material/ListItemButton";
import ListItemIcon from "@mui/material/ListItemIcon";
import ListItemText from "@mui/material/ListItemText";
import Switch from "@mui/material/Switch";

import { DarkMode, Palette } from "@mui/icons-material";

import { ColorMode, useColorMode } from "../../../services/colorMode";

export default function AppearanceSettings() {
  const { t } = useTranslation();

  const [colorMode, toggle] = useColorMode();

  return (
    <Box mb={5} mt={5}>
      <Box mb={2} display='flex' justifyContent='start' pl={2}>
        <Box marginY='auto' mt={1.5} marginRight={4}>
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
