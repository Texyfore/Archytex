import { Language } from "@mui/icons-material";
import { IconButton, Menu, MenuItem, Tooltip, Typography } from "@mui/material";
import React from "react";

//TODO: Get language options from api
const languageOptions = [
  {
    id: "en",
    name: "🇬🇧 English",
  },
  { id: "hu", name: "🇭🇺 Magyar" },
  { id: "jp", name: "🇯🇵 日本語" },
];

const languageMenuHeight = 48;

interface LanguageMenuProps {
  open: boolean;
  handleClick: (event: React.MouseEvent<HTMLElement>) => void;
  handleClose: (event: {}, reason: "backdropClick" | "escapeKeyDown") => void;
  anchorEl: Element | ((element: Element) => Element) | null | undefined;
}

export default function LanguageSelectDropdown({
  open,
  handleClick,
  handleClose,
  anchorEl,
}: LanguageMenuProps) {
  return (
    <React.Fragment>
      <Tooltip title={"Select language"}>
        <IconButton
          aria-label='more'
          id='long-button'
          aria-controls='long-menu'
          aria-expanded={open ? "true" : undefined}
          aria-haspopup='true'
          onClick={handleClick}
        >
          <Language />
        </IconButton>
      </Tooltip>
      <Menu
        anchorEl={anchorEl}
        open={open}
        onClose={handleClose}
        PaperProps={{
          style: {
            maxHeight: languageMenuHeight * 4.5,
            width: "20ch",
          },
        }}
      >
        {languageOptions.map((option) => (
          //TODO: Handle language change on menu item click
          <MenuItem key={option.id} onClick={() => console.log(option.name)}>
            <Typography>
              {`${option.name}\t(${option.id.toUpperCase()})`}
            </Typography>
          </MenuItem>
        ))}
      </Menu>
    </React.Fragment>
  );
}
