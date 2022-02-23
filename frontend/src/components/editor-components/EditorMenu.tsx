import React, { useState } from "react";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import List from "@mui/material/List";
import ListItem from "@mui/material/ListItem";

import { Settings } from "@mui/icons-material";

import LibraryDialog from "./library/LibraryDialog";

import { ColorMode, useColorMode } from "../../services/colorMode";

type libraryType = "textureLibrary" | "propLibrary";

interface Props {
  libraryType: libraryType;
}
export default function EditorMenu({ libraryType }: Props) {
  const { t } = useTranslation();

  const [colorMode, _] = useColorMode();

  //Library dialog
  const [libraryOpen, setLibraryOpen] = useState<boolean>(false);
  const handleLibraryClickOpen = () => {
    setLibraryOpen(true);
  };
  const handleLibraryClose = () => {
    setLibraryOpen(false);
  };

  return (
    <>
      <Box
        width='400px'
        display='flex'
        flexDirection='column'
        borderLeft={
          colorMode === ColorMode.Dark
            ? "1px solid #2E2E2E"
            : "1px solid #BABABA"
        }
      >
        {/* Settings */}
        <Box
          borderBottom={
            colorMode === ColorMode.Dark
              ? "1px solid #2E2E2E"
              : "1px solid #BABABA"
          }
          display='flex'
          alignItems='center'
        >
          <Settings
            sx={{
              marginLeft: 2,
            }}
          />
          <Typography marginY={1} marginLeft={1}>
            {t("settings")}
          </Typography>
        </Box>
        <Box sx={{ overflowY: "scroll" }}>
          <List>
            <ListItem onClick={handleLibraryClickOpen}>
              {libraryType === "textureLibrary"
                ? t("choose_texture")
                : libraryType === "propLibrary"
                ? t("choose_prop")
                : ""}
            </ListItem>
          </List>
        </Box>
      </Box>

      {/* Library dialog */}
      <LibraryDialog
        open={libraryOpen}
        handleClose={handleLibraryClose}
        libraryType={libraryType}
      />
    </>
  );
}
