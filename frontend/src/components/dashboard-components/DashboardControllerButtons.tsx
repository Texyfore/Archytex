import {
  Home,
  PlayCircleOutlined,
  Settings,
  Source,
} from "@mui/icons-material";
import {
  List,
  ListItemButton,
  ListItemIcon,
  ListItemText,
} from "@mui/material";
import React from "react";
import { useTranslation } from "react-i18next";
import { useHistory } from "react-router-dom";
import { SubPage, useSubPage } from "../../services/selectedDashboardSubPage";

export default function DashboardControllerButtons() {
  const { t } = useTranslation();
  const buttonList: { text: string; icon: JSX.Element; id: SubPage }[] = [
    {
      text: t("projects"),
      icon: <Source sx={{ fontSize: { lg: 30, xl: 32 } }} />,
      id: "projects" as SubPage,
    },
    // {
    //   text: "Community portfolio",
    //   icon: <ColorLens sx={{ fontSize: { lg: 30, xl: 32 } }} />,
    // },
    {
      text: t("settings"),
      icon: <Settings sx={{ fontSize: { lg: 30, xl: 32 } }} />,
      id: "settings" as SubPage,
    },
  ];
  const [page, dispatch] = useSubPage();
  const handleListItemClick = (
    event: React.MouseEvent<HTMLDivElement, MouseEvent>,
    id: SubPage
  ) => {
    dispatch(id);
  };

  const history = useHistory();

  return (
    <List sx={{ marginX: { lg: 6 } }}>
      <ListItemButton
        sx={{
          paddingX: { lg: 3 },
          paddingY: { lg: 2 },
          marginTop: { sm: 2, lg: 1 },
          marginBottom: 2,
          borderRadius: "2px",
          border: ".5px solid white",
        }}
        key={0}
      >
        <ListItemIcon sx={{ paddingLeft: { lg: 0, xl: 2 } }}>
          <PlayCircleOutlined sx={{ fontSize: { lg: 30, xl: 35 } }} />
        </ListItemIcon>
        <ListItemText
          sx={{ marginLeft: { lg: 0, xl: 8 } }}
          primary={t("launch_archytex")}
          primaryTypographyProps={{
            fontSize: { lg: "12pt", xl: "15pt" },
          }}
          onClick={() => history.push("/editor")}
        />
      </ListItemButton>
      {buttonList.map((props, index) => (
        <ListItemButton
          sx={{
            paddingX: { lg: 3 },
            paddingY: { lg: 2 },
            marginY: { lg: 1 },
            borderRadius: "2px",
          }}
          key={index + 1}
          selected={page === props.id}
          onClick={(event) => handleListItemClick(event, props.id)}
        >
          <ListItemIcon sx={{ paddingLeft: { lg: 0, xl: 2 } }}>
            {props.icon}
          </ListItemIcon>
          <ListItemText
            sx={{ marginLeft: { lg: 0, xl: 8 } }}
            primary={props.text}
            primaryTypographyProps={{
              fontSize: { lg: "12pt", xl: "15pt" },
            }}
          />
        </ListItemButton>
      ))}
      <ListItemButton
        sx={{
          display: { xs: "flex", md: "none" },
          paddingX: { lg: 3 },
          paddingY: { lg: 2 },
          marginY: { lg: 1 },
          borderRadius: "2px",
        }}
        key={99}
        onClick={() => history.push("/")}
      >
        <ListItemIcon sx={{ paddingLeft: { lg: 0, xl: 2 } }}>
          <Home />
        </ListItemIcon>
        <ListItemText
          sx={{ marginLeft: { lg: 0, xl: 8 } }}
          primary={t("home")}
          primaryTypographyProps={{
            fontSize: { lg: "12pt", xl: "15pt" },
          }}
        />
      </ListItemButton>
    </List>
  );
}
