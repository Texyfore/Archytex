import React, { useEffect, useState } from "react";

import { useTranslation } from "react-i18next";

import Draggable from "react-draggable";
import TextureLibrary from "./TextureLibrary";
import PropLibrary from "./PropLibrary";
import SearchBar from "../../general-components/SearchBar";
import Dialog from "@mui/material/Dialog";
import IconButton from "@mui/material/IconButton";
import DialogTitle from "@mui/material/DialogTitle";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import DialogContent from "@mui/material/DialogContent";
import DialogActions from "@mui/material/DialogActions";
import Tooltip from "@mui/material/Tooltip";
import Paper, { PaperProps } from "@mui/material/Paper";

import { Close, FilterList } from "@mui/icons-material";

import FilterMenu from "./FilterMenu";

import { Prop, Texture } from "../../../services/Library";

function PaperComponent(props: PaperProps) {
  return (
    <Draggable
      handle='#draggable-dialog-title'
      cancel={'[class*="MuiDialogContent-root"]'}
    >
      <Paper {...props} />
    </Draggable>
  );
}

type LibraryType = "textureLibrary" | "propLibrary";

interface Props {
  open: boolean;
  handleClose: () => void;
  libraryType: LibraryType;
  texture: Texture;
  handleTextureChange: (texture: Texture) => void;
  prop: Prop;
  handlePropChange: (prop: Prop) => void;
  textures: Texture[];
  props: Prop[];
}

export default function LibraryDialog({
  open,
  handleClose,
  libraryType,
  texture,
  handleTextureChange,
  prop,
  handlePropChange,
  textures,
  props,
}: Props) {
  //Translation
  const { t } = useTranslation();
  const tooltipText = t("select_an_item_to_use");

  //Dialog
  const descriptionElementRef = React.useRef<HTMLElement>(null);
  React.useEffect(() => {
    if (open) {
      const { current: descriptionElement } = descriptionElementRef;
      if (descriptionElement !== null) {
        descriptionElement.focus();
      }
      if (libraryType === "textureLibrary") {
        handleTextureSelectionChange(texture);
      } else if (libraryType === "propLibrary") {
        handlePropSelectionChange(prop);
      }
    }
  }, [open, libraryType, texture, prop]);

  // Texture selection handling
  const [selectedTexture, setSelectedTexture] = useState<Texture | undefined>(
    undefined
  );
  const handleTextureSelectionChange = (item: Texture | undefined) => {
    setSelectedTexture(item);
  };

  // Prop selection handling
  const [selectedProp, setSelectedProp] = useState<Prop | undefined>(undefined);
  const handlePropSelectionChange = (item: Prop | undefined) => {
    setSelectedProp(item);
  };

  //Apply new item
  const handleApplyNewItem = () => {
    if (libraryType === "textureLibrary" && selectedTexture !== undefined) {
      handleTextureChange(selectedTexture);
    } else if (libraryType === "propLibrary" && selectedProp !== undefined) {
      handlePropChange(selectedProp);
    }

    handleClose();
  };

  //Filter menu
  const [anchorEl, setAnchorEl] = React.useState<null | HTMLElement>(null);
  const openFilterMenu = Boolean(anchorEl);
  const handleFilterClick = (event: React.MouseEvent<HTMLButtonElement>) => {
    setAnchorEl(event.currentTarget);
  };
  const handleFilterMenuClose = () => {
    setAnchorEl(null);
  };

  //Filter menu items
  const categorySet: Set<string> = new Set<string>();
  libraryType === "textureLibrary"
    ? textures.forEach((texture) => {
      texture.categories.forEach((category) =>
        categorySet.add(category)
      );
    })
    : props.forEach((prop) => {
      prop.categories.forEach((category) =>
        categorySet.add(category)
      );
    });

  const categories: string[] = Array.from(categorySet.values());

  const [checkedCategories, setCheckedCategories] =
    React.useState<string[]>(categories);

  const handleToggleCategory = (category: string) => () => {
    let newChecked = [...checkedCategories];
    checkedCategories.some((c) => c === category)
      ? (newChecked = newChecked.filter((c) => c !== category))
      : newChecked.push(category);
    setCheckedCategories(newChecked);
  };

  const handleToggleAll = () => {
    checkedCategories.length !== categories.length
      ? setCheckedCategories(categories)
      : setCheckedCategories([]);
  };

  // Seach bar
  const [query, setQuery] = useState("");
  const handleQueryChange = (query: string) => {
    setQuery(query);
    handleTextureSelectionChange(undefined);
    handlePropSelectionChange(undefined);
  };

  // Reset dialog on every open
  useEffect(() => {
    if (open) {
      setCheckedCategories(categories);
      setQuery("");
    }
  }, [open]);

  return (
    <Dialog
      open={open}
      onClose={handleClose}
      scroll='paper'
      PaperComponent={PaperComponent}
      PaperProps={{ style: { minHeight: "95%", maxHeight: "95%" } }}
    >
      <IconButton
        onClick={handleClose}
        sx={{
          position: "absolute",
          right: 8,
          top: 8,
          color: (theme) => theme.palette.grey[500],
        }}
      >
        <Close />
      </IconButton>
      <DialogTitle
        style={{ cursor: "move", borderBottom: "1px solid grayText" }}
        id='draggable-dialog-title'
      >
        <Box display='flex' flexWrap='wrap' marginBottom={2}>
          <Typography variant='h6'>
            {libraryType === "textureLibrary"
              ? t("texture_library")
              : libraryType === "propLibrary"
                ? t("prop_library")
                : ""}
          </Typography>
        </Box>
        <Box display='flex' justifyContent='space-between'>
          <Button
            endIcon={<FilterList />}
            color='inherit'
            onClick={handleFilterClick}
          >
            {t("filter_items")}
          </Button>
          <SearchBar query={query} handleQueryChange={handleQueryChange} />
        </Box>
      </DialogTitle>
      <DialogContent>
        <Box width={550}>
          {libraryType === "textureLibrary" ? (
            <TextureLibrary
              query={query}
              checkedCategories={checkedCategories}
              selected={selectedTexture}
              handleSelectionChange={handleTextureSelectionChange}
              textures={textures}
            />
          ) : (
            <PropLibrary
              query={query}
              checkedCategories={checkedCategories}
              selected={selectedProp}
              handleSelectionChange={handlePropSelectionChange}
              props={props}
            />
          )}
        </Box>
      </DialogContent>
      <DialogActions>
        <Box display={selectedTexture === undefined ? "block" : "none"}>
          <Tooltip title={tooltipText} followCursor>
            <span>
              <Button
                onClick={handleClose}
                disabled={selectedTexture === undefined}
              >
                {t("accept")}
              </Button>
            </span>
          </Tooltip>
        </Box>
        <Box display={selectedTexture !== undefined ? "block" : "none"}>
          <Button
            onClick={handleApplyNewItem}
            disabled={selectedTexture === undefined}
          >
            {t("accept")}
          </Button>
        </Box>
      </DialogActions>

      {/* Filter Menu */}
      <FilterMenu
        open={openFilterMenu}
        anchorEl={anchorEl}
        handleClose={handleFilterMenuClose}
        categories={categories}
        checkedCategories={checkedCategories}
        onCheck={handleToggleCategory}
        toggleAll={handleToggleAll}
      />
    </Dialog>
  );
}
