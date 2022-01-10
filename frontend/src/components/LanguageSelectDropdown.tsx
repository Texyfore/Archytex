import { Language } from "@mui/icons-material";
import { IconButton, Menu, MenuItem, Tooltip, Typography } from "@mui/material";
import { changeLanguage } from "i18next";
import React from "react";
import { useTranslation } from "react-i18next";

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
  const { t } = useTranslation();
  const tooltipText: string = t("select_language");

  const handleChangeLanguage = (
    id: string,
    handleClose: (event: {}, reason: "backdropClick" | "escapeKeyDown") => void
  ) => {
    changeLanguage(id);
    handleClose({}, "backdropClick");
  };

  return (
    <React.Fragment>
      <Tooltip title={tooltipText}>
        <IconButton
          aria-expanded={open ? "true" : undefined}
          aria-haspopup="true"
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
          <MenuItem
            key={option.id}
            onClick={() => handleChangeLanguage(option.id, handleClose)}
          >
            <Typography>
              {`${option.name}\t(${option.id.toUpperCase()})`}
            </Typography>
          </MenuItem>
        ))}
      </Menu>
    </React.Fragment>
  );
}
