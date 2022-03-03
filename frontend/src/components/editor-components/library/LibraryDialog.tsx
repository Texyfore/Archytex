import React, { useState } from "react";

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

import Prop from "../../../services/types/Prop";
import Texture from "../../../services/types/Texture";
import Category from "../../../services/libraries/Category";
import getTextureCategories from "../../../services/libraries/TextureCategories";
import getPropCategories from "../../../services/libraries/PropCategories";

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
}

export default function LibraryDialog({
  open,
  handleClose,
  libraryType,
  texture,
  handleTextureChange,
  prop,
  handlePropChange,
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
      handleSelectionChange(libraryType === "textureLibrary" ? texture : prop);
    }
  }, [open, libraryType, texture, prop]);

  //Selection handling
  const [selected, setSelected] = useState<Texture | Prop | undefined>(
    undefined
  );
  const handleSelectionChange = (item: Texture | Prop | undefined) => {
    setSelected(item);
  };

  //Apply new item
  const handleApplyNewItem = () => {
    if (selected !== undefined) {
      libraryType === "textureLibrary"
        ? handleTextureChange(selected)
        : handlePropChange(selected);
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
  const categories =
    libraryType === "textureLibrary"
      ? getTextureCategories()
      : getPropCategories();
  const [checkedCategories, setCheckedCategories] =
    React.useState<Category[]>(categories);
  const handleToggleCategory = (category: Category) => () => {
    let newChecked = [...checkedCategories];
    checkedCategories.some((c) => c.id === category.id)
      ? (newChecked = newChecked.filter((c) => c.id !== category.id))
      : newChecked.push(category);
    setCheckedCategories(newChecked);
  };
  const handleToggleAll = () => {
    checkedCategories.length !== categories.length
      ? setCheckedCategories(categories)
      : setCheckedCategories([]);
  };

  //Seach bar
  const [query, setQuery] = useState("");
  const handleQueryChange = (query: string) => {
    setQuery(query);
    handleSelectionChange(undefined);
  };

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
              : t("library")}
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
              selected={selected}
              handleSelectionChange={handleSelectionChange}
            />
          ) : (
            <PropLibrary
              query={query}
              checkedCategories={checkedCategories}
              selected={selected}
              handleSelectionChange={handleSelectionChange}
            />
          )}
        </Box>
      </DialogContent>
      <DialogActions>
        <Box display={selected === undefined ? "block" : "none"}>
          <Tooltip title={tooltipText} followCursor>
            <span>
              <Button onClick={handleClose} disabled={selected === undefined}>
                {t("accept")}
              </Button>
            </span>
          </Tooltip>
        </Box>
        <Box display={selected !== undefined ? "block" : "none"}>
          <Button
            onClick={handleApplyNewItem}
            disabled={selected === undefined}
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
