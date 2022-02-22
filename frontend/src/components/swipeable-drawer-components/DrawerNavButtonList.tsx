import React from "react";

import { Link as L } from "react-router-dom";

import { useTranslation } from "react-i18next";

import List from "@mui/material/List";
import ListItemButton from "@mui/material/ListItemButton";
import ListItemIcon from "@mui/material/ListItemIcon";
import ListItemText from "@mui/material/ListItemText";

import {
  DashboardRounded,
  Home,
  InfoOutlined,
  Login,
} from "@mui/icons-material";

import { useApi } from "../../services/user/api";

interface NavButton {
  text: string;
  icon: JSX.Element;
  route: string;
}
interface Props {
  handleClose: () => void;
}
export default function DrawerNavButtonList({ handleClose }: Props) {
  const { t } = useTranslation();

  const api = useApi();

  const navButtons: NavButton[] = [
    {
      text: t("home"),
      icon: <Home />,
      route: "/",
    },
    {
      text: t("login"),
      icon: <Login />,
      route: "/login",
    },
    {
      text: t("about"),
      icon: <InfoOutlined />,
      route: "/about",
    },
  ];
  const loggedInnavButtons: NavButton[] = [
    {
      text: t("home"),
      icon: <Home />,
      route: "/",
    },
    {
      text: t("dashboard"),
      icon: <DashboardRounded />,
      route: "/dashboard",
    },
    {
      text: t("about"),
      icon: <InfoOutlined />,
      route: "/about",
    },
  ];

  const buttonList: NavButton[] =
    api?.state === "logged-in" ? loggedInnavButtons : navButtons;

  return (
    <List>
      {buttonList.map((props) => (
        <ListItemButton
          component={L}
          to={props.route}
          onClick={handleClose}
          sx={{
            borderRadius: "2px",
          }}
          key={props.text}
        >
          <ListItemIcon>{props.icon}</ListItemIcon>
          <ListItemText primary={props.text} />
        </ListItemButton>
      ))}
    </List>
  );
}
