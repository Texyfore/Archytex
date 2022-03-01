import React from "react";

import { useTranslation } from "react-i18next";

import Checkbox from "@mui/material/Checkbox";
import List from "@mui/material/List";
import ListItem from "@mui/material/ListItem";
import ListItemButton from "@mui/material/ListItemButton";
import ListItemIcon from "@mui/material/ListItemIcon";
import ListItemText from "@mui/material/ListItemText";
import Menu from "@mui/material/Menu";
import Divider from "@mui/material/Divider";

import Category from "../../../services/libraries/Category";

interface Props {
  anchorEl: null | HTMLElement;
  open: boolean;
  handleClose: () => void;
  categories: Category[];
  checkedCategories: Category[];
  onCheck: (category: Category) => () => void;
  toggleAll: () => void;
}

export default function FilterMenu({
  anchorEl,
  open,
  handleClose,
  categories,
  checkedCategories,
  onCheck,
  toggleAll,
}: Props) {
  const { t } = useTranslation();

  return (
    <Menu
      anchorEl={anchorEl}
      open={open}
      onClose={handleClose}
      PaperProps={{
        style: {
          maxHeight: 300,
          width: 250,
        },
      }}
    >
      <List>
        <ListItem key={0} disablePadding>
          <ListItemButton role={undefined} onClick={toggleAll} dense>
            <ListItemIcon>
              <Checkbox
                edge='start'
                checked={checkedCategories.length === categories.length}
                tabIndex={-1}
              />
            </ListItemIcon>
            <ListItemText primary={t("select_all")} />
          </ListItemButton>
        </ListItem>
        <Divider />
        {categories.map((category, index) => (
          <ListItem key={index + 1} disablePadding>
            <ListItemButton role={undefined} onClick={onCheck(category)} dense>
              <ListItemIcon>
                <Checkbox
                  edge='start'
                  checked={checkedCategories.some((c) => c.id === category.id)}
                  tabIndex={-1}
                />
              </ListItemIcon>
              <ListItemText
                primary={
                  category.name.charAt(0).toUpperCase() +
                  category.name.toLocaleLowerCase().slice(1)
                }
              />
            </ListItemButton>
          </ListItem>
        ))}
      </List>
    </Menu>
  );
}
