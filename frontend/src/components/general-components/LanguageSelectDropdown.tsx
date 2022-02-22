import React from "react";

import { useTranslation } from "react-i18next";
import { changeLanguage } from "i18next";

import ReactCountryFlag from "react-country-flag";

import Menu from "@mui/material/Menu";
import MenuItem from "@mui/material/MenuItem";
import Tooltip from "@mui/material/Tooltip";
import IconButton from "@mui/material/IconButton";
import Typography from "@mui/material/Typography";

import { Language } from "@mui/icons-material";

const languageOptions: { id: string; name: string }[] = [
  {
    id: "gb",
    name: "English",
  },
  { id: "hu", name: "Magyar" },
  { id: "jp", name: "日本語" },
];

export default function LanguageSelectDropdown() {
  const { t } = useTranslation();
  const tooltipText: string = t("select_language");
  const handleChangeLanguage = (
    id: string,
    handleClose: (event: {}, reason: "backdropClick" | "escapeKeyDown") => void
  ) => {
    changeLanguage(id);
    handleClose({}, "backdropClick");
  };

  const [anchorEl, setAnchorEl] = React.useState<null | HTMLElement>(null);
  const open = Boolean(anchorEl);
  const handleClick = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };
  const handleClose = () => {
    setAnchorEl(null);
  };

  return (
    <>
      <Tooltip title={tooltipText}>
        <IconButton onClick={handleClick} sx={{ width: 42, height: 42 }}>
          <Language />
        </IconButton>
      </Tooltip>

      <Menu
        anchorEl={anchorEl}
        open={open}
        onClose={handleClose}
        PaperProps={{
          style: {
            maxHeight: 45 * 4.5,
            width: "150px",
          },
        }}
      >
        {languageOptions.map((option) => (
          <MenuItem
            key={option.id}
            onClick={() => handleChangeLanguage(option.id, handleClose)}
          >
            <ReactCountryFlag
              countryCode={option.id}
              style={{
                fontSize: "16pt",
                marginRight: 10,
                textShadow: "0px 0px 2px grey",
              }}
            />
            <Typography>
              {`${option.name}\t(${option.id.toUpperCase()})`}
            </Typography>
          </MenuItem>
        ))}
      </Menu>
    </>
  );
}
