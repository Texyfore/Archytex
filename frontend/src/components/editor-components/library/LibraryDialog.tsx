import React from "react";
import {
  Box,
  Button,
  Checkbox,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  IconButton,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Menu,
  Paper,
  PaperProps,
  Typography,
} from "@mui/material";
import { Close, FilterList } from "@mui/icons-material";
import Draggable from "react-draggable";
import TextureLibrary from "./TextureLibrary";
import SearchBar from "../../SearchBar";

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

type LibraryType = "textureLibrary" | "propLibrary" | "projectLibrary";

interface LibraryDialogProps {
  open: boolean;
  handleClose: () => void;
  libraryType: LibraryType;
}

export default function LibraryDialog({
  open,
  handleClose,
  libraryType,
}: LibraryDialogProps) {
  //Dialog
  const descriptionElementRef = React.useRef<HTMLElement>(null);
  React.useEffect(() => {
    if (open) {
      const { current: descriptionElement } = descriptionElementRef;
      if (descriptionElement !== null) {
        descriptionElement.focus();
      }
    }
  }, [open]);

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

  return (
    <>
      <Dialog
        open={open}
        onClose={handleClose}
        scroll='paper'
        PaperComponent={PaperComponent}
        aria-labelledby='draggable-dialog-title'
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
                ? "Texture library"
                : libraryType === "propLibrary"
                ? "Prop library"
                : libraryType === "projectLibrary"
                ? "Project library"
                : "Library"}
            </Typography>
          </Box>
          <Box
            sx={{
              position: "absolute",
              right: 60,
              top: 10,
            }}
          >
            <Button
              endIcon={<FilterList />}
              color='inherit'
              onClick={handleFilterClick}
            >
              Filter results
            </Button>
          </Box>
          <SearchBar />
        </DialogTitle>
        <DialogContent>
          <Box width={550}>
            <TextureLibrary />
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleClose}>Accept</Button>
        </DialogActions>
      </Dialog>

      {/* Filter Menu */}
      <Menu
        id='basic-menu'
        anchorEl={anchorEl}
        open={openFilterMenu}
        onClose={handleFilterMenuClose}
        sx={{ maxHeight: 500, overflowY: "scroll" }}
      >
        <List>
          {[...new Array(10)].map((_, index) => (
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
                <ListItemText primary={`Filter item ${index + 1}`} />
              </ListItemButton>
            </ListItem>
          ))}
        </List>
      </Menu>
    </>
  );
}
