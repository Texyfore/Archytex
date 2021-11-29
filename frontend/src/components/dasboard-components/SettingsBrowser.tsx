import React from "react";
import {
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Switch,
  Typography,
} from "@mui/material";
import { styled } from "@mui/material/styles";
import { DarkMode, House } from "@mui/icons-material";

const headerHeight = 50;
const SettingList = styled(List)(({ theme }) => ({
  border: "none",
  overflowY: "scroll",
  height: `calc(100vh - 56px - ${headerHeight}px)`,
  [`${theme.breakpoints.up("xs")} and (orientation: landscape)`]: {
    height: `calc(100vh - 48px - ${headerHeight}px)`,
  },
  [theme.breakpoints.up("sm")]: {
    height: `calc(100vh - 64px - ${headerHeight}px)`,
  },
  [theme.breakpoints.up("lg")]: {
    height: `calc(100% - ${headerHeight}px)`,
  },
}));

interface setting {
  name: string;
  description?: string;
  type: "switch" | "text";
  icon: JSX.Element;
}

export default function SettingsBrowser() {
  const settings: setting[] = [
    {
      name: "Dark mode",
      description: "Set the application theme to dark.",
      type: "switch",
      icon: <DarkMode />,
    },
    {
      name: "Random input setting",
      description: "Description of random input setting",
      type: "text",
      icon: <House />,
    },
    {
      name: "Random input setting",
      description: "Description of random input setting",
      type: "text",
      icon: <House />,
    },
    {
      name: "Random input setting",
      description: "Description of random input setting",
      type: "text",
      icon: <House />,
    },
    {
      name: "Random input setting",
      description: "Description of random input setting",
      type: "text",
      icon: <House />,
    },
    {
      name: "Random input setting",
      description: "Description of random input setting",
      type: "text",
      icon: <House />,
    },
    {
      name: "Random input setting",
      description: "Description of random input setting",
      type: "text",
      icon: <House />,
    },
    {
      name: "Random input setting",
      description: "Description of random input setting",
      type: "text",
      icon: <House />,
    },
    {
      name: "Random input setting",
      description: "Description of random input setting",
      type: "text",
      icon: <House />,
    },
    {
      name: "Random input setting",
      description: "Description of random input setting",
      type: "text",
      icon: <House />,
    },
    {
      name: "Random input setting",
      description: "Description of random input setting",
      type: "text",
      icon: <House />,
    },
    {
      name: "Random input setting",
      description: "Description of random input setting",
      type: "text",
      icon: <House />,
    },
    {
      name: "Random input setting",
      description: "Description of random input setting",
      type: "text",
      icon: <House />,
    },
    {
      name: "Random input setting",
      description: "Description of random input setting",
      type: "text",
      icon: <House />,
    },
    {
      name: "Random input setting",
      description: "Description of random input setting",
      type: "text",
      icon: <House />,
    },
    {
      name: "Random input setting",
      description: "Description of random input setting",
      type: "text",
      icon: <House />,
    },
    {
      name: "Random input setting",
      description: "Description of random input setting",
      type: "text",
      icon: <House />,
    },
    {
      name: "Random input setting",
      description: "Description of random input setting",
      type: "text",
      icon: <House />,
    },
    {
      name: "Random input setting",
      description: "Description of random input setting",
      type: "text",
      icon: <House />,
    },
    {
      name: "Random input setting",
      description: "Description of random input setting",
      type: "text",
      icon: <House />,
    },
    {
      name: "Random input setting",
      description: "Description of random input setting",
      type: "text",
      icon: <House />,
    },
  ];

  const [checked, setChecked] = React.useState(["wifi"]);

  const handleToggle = (value: string) => () => {
    const currentIndex = checked.indexOf(value);
    const newChecked = [...checked];

    if (currentIndex === -1) {
      newChecked.push(value);
    } else {
      newChecked.splice(currentIndex, 1);
    }

    setChecked(newChecked);
  };
  //TODO: Use scrollspy
  // https://codesandbox.io/s/material-demo-xu80m?file=/ScrollSpyTabs.js
  return (
    <SettingList>
      {settings.map((setting) => (
        <ListItem key={setting.name} disablePadding>
          <ListItemButton
            onClick={handleToggle(setting.name)}
            sx={{ borderRadius: 2 }}
          >
            <ListItemIcon>{setting.icon}</ListItemIcon>
            <ListItemText
              primary={<Typography>{setting.name}</Typography>}
              secondary={setting.description}
            />
            <Switch
              edge='end'
              onChange={handleToggle(setting.name)}
              checked={checked.indexOf(setting.name) !== -1}
              inputProps={{
                "aria-labelledby": "switch-list-label-bluetooth",
              }}
            />
          </ListItemButton>
        </ListItem>
      ))}
    </SettingList>
  );
}
