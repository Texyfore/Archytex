import React, { useState } from "react";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import Button from "@mui/material/Button";
import Typography from "@mui/material/Typography";
import Divider from "@mui/material/Divider";

import { Chair, Settings, Texture as TextureIcon } from "@mui/icons-material";

import LibraryDialog from "./library/LibraryDialog";

import { ColorMode, useColorMode } from "../../services/colorMode";
import { Prop, Texture } from "../../services/Library";

import ReactImageFallback from "react-image-fallback";
import Environment from "../../env";

type LibraryType = "textureLibrary" | "propLibrary";

interface Props {
  texture: Texture;
  handleTextureChange: (texture: Texture) => void;
  prop: Prop;
  handlePropChange: (prop: Prop) => void;
  textures: Texture[];
  props: Prop[];
}

export default function EditorMenu({
  texture,
  handleTextureChange,
  prop,
  handlePropChange,
  textures,
  props,
}: Props) {
  const { t } = useTranslation();

  const [colorMode, _] = useColorMode();

  //Library dialog
  const [libraryType, setLibraryType] = useState<LibraryType>("textureLibrary");
  const [libraryOpen, setLibraryOpen] = useState(false);
  const handleLibraryClickOpen = (type: LibraryType) => {
    setLibraryType(type);
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

        <Box display='flex' p={1}>
          <TextureIcon />
          <Typography ml={1}>{t("texture")}</Typography>
        </Box>
        <Box p={1} mb={1} display='flex'>
          <Box width={100} height={100} mr={2}>
            <ReactImageFallback
              src={`${Environment.asset_url}/thumbnails/${texture.name}.webp`}
              fallbackImage={require("../../img/unknown.webp").default}
              alt='texture'
              style={{ objectFit: "cover", borderRadius: 2 }}
              height='100%'
              width='100%'
            />
          </Box>
          <Box
            display='flex'
            flexDirection='column'
            justifyContent='center'
            gap={1}
          >
            <Typography variant='caption'>{t("selected_texture")}</Typography>
            <Typography>
              {texture.name.charAt(0).toUpperCase() +
                texture.name.replaceAll("_", " ").slice(1)}
            </Typography>
            <Button
              variant='outlined'
              onClick={() => handleLibraryClickOpen("textureLibrary")}
            >
              {t("choose_texture")}
            </Button>
          </Box>
        </Box>

        <Divider />

        <Box display='flex' p={1}>
          <Chair />
          <Typography ml={1}>{t("prop")}</Typography>
        </Box>
        <Box p={1} display='flex'>
          <Box width={100} height={100} borderRadius={1} mr={2}>
            <ReactImageFallback
              src={`${Environment.asset_url}/thumbnails/${prop.name}.webp`}
              fallbackImage={require("../../img/unknown.webp").default}
              alt='prop'
              style={{ objectFit: "contain", borderRadius: 2 }}
              height='100%'
              width='100%'
            />
          </Box>
          <Box
            display='flex'
            flexDirection='column'
            justifyContent='center'
            gap={1}
          >
            <Typography variant='caption'>{t("selected_prop")}</Typography>
            <Typography>
              {prop.name.charAt(0).toUpperCase() +
                prop.name.replaceAll("_", " ").slice(1)}
            </Typography>
            <Button
              variant='outlined'
              onClick={() => handleLibraryClickOpen("propLibrary")}
            >
              {t("choose_prop")}
            </Button>
          </Box>
        </Box>
      </Box>

      {/* Library dialog */}
      <LibraryDialog
        libraryType={libraryType}
        open={libraryOpen}
        handleClose={handleLibraryClose}
        texture={texture}
        handleTextureChange={handleTextureChange}
        prop={prop}
        handlePropChange={handlePropChange}
        textures={textures}
        props={props}
      />
    </>
  );
}
