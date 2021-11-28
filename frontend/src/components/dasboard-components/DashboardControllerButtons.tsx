import {
  ColorLens,
  PlayCircleOutlined,
  Settings,
  Source,
} from "@mui/icons-material";
import { List, ListItem, ListItemIcon, ListItemText } from "@mui/material";
import React from "react";

export default function DashboardControllerButtons() {
  const buttonList = [
    {
      text: "Launch Archytex",
      icon: <PlayCircleOutlined sx={{ fontSize: { lg: 30, xl: 35 } }} />,
    },
    {
      text: "Projects",
      icon: <Source sx={{ fontSize: { lg: 30, xl: 32 } }} />,
    },
    {
      text: "Community portfolio",
      icon: <ColorLens sx={{ fontSize: { lg: 30, xl: 32 } }} />,
    },
    {
      text: "Settings",
      icon: <Settings sx={{ fontSize: { lg: 30, xl: 32 } }} />,
    },
  ];

  const [selectedIndex, setSelectedIndex] = React.useState(1);

  const handleListItemClick = (
    event: React.MouseEvent<HTMLDivElement, MouseEvent>,
    index: number
  ) => {
    setSelectedIndex(index);
  };

  return (
    <List sx={{ marginX: { lg: 6 } }}>
      {buttonList.map((props, index) => (
        <ListItem
          sx={{
            paddingX: { lg: 3 },
            paddingY: { lg: 2 },
            marginY: { lg: 1 },
            borderRadius: "2px",
          }}
          button
          key={index}
          selected={selectedIndex === index}
          onClick={(event) => handleListItemClick(event, index)}
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
        </ListItem>
      ))}
    </List>
  );
}
