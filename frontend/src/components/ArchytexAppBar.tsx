import React, { useState } from "react";
import {
  Close,
  MenuOutlined,
} from "@mui/icons-material";
import {
  AppBar,
  IconButton,
  Toolbar,
  Box,
  Button,
} from "@mui/material";
import { styled } from "@mui/material/styles";
import ArchytexLogoWithText from "./ArchytexLogoWithText";
import UserIconButton from "./UserIconButton";
import GeneralSwipeableDrawer from "./GeneralSwipeableDrawer";
import { useHistory } from "react-router-dom";
import { useApi } from "../services/user/api";

const CustomAppBar = styled(AppBar)(({ theme }) => ({
  zIndex: theme.zIndex.drawer + 1,
  filter: "drop-shadow(0px 2px 4px rgba(0,0,0,0.5))",
}));

interface AppBarProps {
  content: "general" | "dashboard"
}

function ArchytexAppBar({ content }: AppBarProps) {
  const api = useApi();
  const history = useHistory();

  const [open, setOpen] = useState(false);
  const handleOpenChange = (value: boolean) => {
    setOpen(value)
  }
  const handleDrawerToggle = () => {
    handleOpenChange(!open);
  };

  return (
    <>
      <CustomAppBar position='fixed' elevation={0}>
        <Toolbar
          sx={{
            justifyContent: "space-between",
            backgroundColor: "background.paper",
          }}
        >
          <Box display={{ xs: "flex", md: "none" }}>
            <IconButton onClick={handleDrawerToggle}>
              {open ? <Close /> : <MenuOutlined />}
            </IconButton>
          </Box>
          <Box width='100%' height='100%'>
            <ArchytexLogoWithText />
          </Box>
          <Box
            marginX={2}
            height='100%'
            display={{ xs: "none", md: "flex" }}
            justifyContent='space-between'
            gap={2}
          >
            <Button color='inherit' variant='text' onClick={()=>history.push("/")}>
              Home
            </Button>
            {api?.state === "logged-in" ?
            <Button color='inherit' variant='text' onClick={()=>history.push("/dashboard")}>
              Dashboard
            </Button> : null}
          </Box>
          <Box width='100%' height='100%' display='flex' justifyContent='end'>
            {api?.state === "not-logged-in" ?
              <Button variant='outlined' onClick={() => history.push("/login")}>
                Login
              </Button> :
              <UserIconButton />
            }
          </Box>
        </Toolbar>
      </CustomAppBar>
      <GeneralSwipeableDrawer content={content} open={open} handleOpenChange={handleOpenChange} />
    </>
  );
}

export default ArchytexAppBar;
