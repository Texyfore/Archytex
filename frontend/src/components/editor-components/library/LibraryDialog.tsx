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
import Menu from "@mui/material/Menu";
import List from "@mui/material/List";
import ListItem from "@mui/material/ListItem";
import ListItemButton from "@mui/material/ListItemButton";
import ListItemIcon from "@mui/material/ListItemIcon";
import Checkbox from "@mui/material/Checkbox";
import ListItemText from "@mui/material/ListItemText";
import Paper, { PaperProps } from "@mui/material/Paper";

import { Close, FilterList } from "@mui/icons-material";

import Prop from "../../../services/types/Prop";
import Texture from "../../../services/types/Texture";

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

enum TextureFilterOptions {
  brick = "Brick",
  wood = "Wood",
  concrete = "Concrete",
  rock = "Rock",
  dirty = "Dirty",
  clean = "Clean",
}
enum PropFilterOptions {
  furniture = "Furniture",
  decoration = "Decoration",
  table = "Table",
  chair = "Chair",
}

type LibraryType = "textureLibrary" | "propLibrary";

interface Props {
  open: boolean;
  handleClose: () => void;
  libraryType: LibraryType;
  texture: Texture;
  handleTextureChange: (id: number) => void;
  prop: Prop;
  handlePropChange: (id: number) => void;
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
      handleSelectionChange(
        libraryType === "textureLibrary" ? texture.id : prop.id
      );
    }
  }, [open, libraryType, texture.id, prop.id]);

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
  const [checkedFilterItems, setCheckedFilterItem] = React.useState([0]);
  const handleToggleFilterItem = (value: number) => () => {
    const currentIndex = checkedFilterItems.indexOf(value);
    const newChecked = [...checkedFilterItems];

    if (currentIndex === -1) {
      newChecked.push(value);
    } else {
      newChecked.splice(currentIndex, 1);
    }

    setCheckedFilterItem(newChecked);
  };

  //Selection handling
  const [selected, setSelected] = useState<number | undefined>(undefined);
  const handleSelectionChange = (n: number | undefined) => {
    setSelected(n);
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

  return (
    <>
      <Dialog
        open={open}
        onClose={handleClose}
        scroll='paper'
        PaperComponent={PaperComponent}
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
              {t("filter_results")}
            </Button>
            <SearchBar query='' handleQueryChange={() => {}} />
          </Box>
        </DialogTitle>
        <DialogContent>
          <Box width={550}>
            {libraryType === "textureLibrary" ? (
              <TextureLibrary
                selected={selected}
                handleSelectionChange={handleSelectionChange}
              />
            ) : (
              <PropLibrary
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
      </Dialog>

      {/* Filter Menu */}
      <Menu
        id='basic-menu'
        anchorEl={anchorEl}
        open={openFilterMenu}
        onClose={handleFilterMenuClose}
        PaperProps={{
          style: {
            maxHeight: 300,
            width: 250,
          },
        }}
      >
        <List>
          {(
            Object.keys(
              libraryType === "textureLibrary"
                ? TextureFilterOptions
                : PropFilterOptions
            ) as Array<keyof typeof TextureFilterOptions>
          ).map((filterOption, index) => (
            <ListItem key={index} disablePadding>
              <ListItemButton
                role={undefined}
                onClick={handleToggleFilterItem(index)}
                dense
              >
                <ListItemIcon>
                  <Checkbox
                    edge='start'
                    checked={checkedFilterItems.indexOf(index) !== -1}
                    tabIndex={-1}
                    disableRipple
                  />
                </ListItemIcon>
                <ListItemText
                  primary={
                    filterOption.charAt(0).toUpperCase() + filterOption.slice(1)
                  }
                />
              </ListItemButton>
            </ListItem>
          ))}
        </List>
      </Menu>
    </>
  );
}
