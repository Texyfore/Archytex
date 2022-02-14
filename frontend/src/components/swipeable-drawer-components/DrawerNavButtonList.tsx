import React from "react";
import { useHistory } from "react-router-dom";

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

interface navButton {
  text: string;
  icon: JSX.Element;
  route: string;
}

export default function DrawerNavButtonList() {
  const { t } = useTranslation();

  const api = useApi();

  const buttonList: navButton[] =
    api?.state === "logged-in"
      ? [
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
        ]
      : [
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

  const history = useHistory();
  const handleClick = (route: string) => {
    history.push(route);
  };

  return (
    <List>
      {buttonList.map((props) => (
        <ListItemButton
          sx={{
            borderRadius: "2px",
          }}
          key={props.text}
          onClick={() => handleClick(props.route)}
        >
          <ListItemIcon>{props.icon}</ListItemIcon>
          <ListItemText primary={props.text} />
        </ListItemButton>
      ))}
    </List>
  );
}
