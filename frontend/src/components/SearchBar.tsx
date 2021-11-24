import React from "react";
import { styled, alpha } from "@mui/material/styles";
import { Search } from "@mui/icons-material";
import { InputBase } from "@mui/material";

const SearchDiv = styled("div")(({ theme }) => ({
  position: "relative",
  borderRadius: theme.shape.borderRadius,
  // backgroundColor: alpha(theme.palette.common.white, 0.15),
  backgroundColor: theme.palette.mode === "dark" ? "#1F1F1F" : "#EBE7EC",
  "&:hover": {
    backgroundColor:
      theme.palette.mode === "dark"
        ? alpha("#1F1F1F", 0.5)
        : alpha("#EBE7EC", 0.5),
  },
  marginLeft: 0,
  width: "100%",
  [theme.breakpoints.up("sm")]: {
    marginLeft: theme.spacing(1),
    width: "auto",
  },
}));

const SearchIconWrapper = styled("div")(({ theme }) => ({
  padding: theme.spacing(0, 2),
  height: "100%",
  position: "absolute",
  pointerEvents: "none",
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
}));

const StyledInputBase = styled(InputBase)(({ theme }) => ({
  color: "inherit",
  "& .MuiInputBase-input": {
    padding: theme.spacing(1, 1, 1, 0),
    // vertical padding + font size from searchIcon
    paddingLeft: `calc(1em + ${theme.spacing(4)})`,
    transition: theme.transitions.create("width"),
    width: "100%",
    [theme.breakpoints.up("sm")]: {
      width: "20ch",
      "&:focus": {
        width: "30ch",
      },
    },
  },
}));

export default function SearchBar() {
  return (
    <SearchDiv>
      <SearchIconWrapper>
        <Search />
      </SearchIconWrapper>
      <StyledInputBase
        placeholder='Search…'
        inputProps={{ "aria-label": "search" }}
      />
    </SearchDiv>
  );
}
