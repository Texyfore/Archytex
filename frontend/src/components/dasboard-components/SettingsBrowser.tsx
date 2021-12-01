import React from "react";
import {
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Switch,
  Typography,
  Box,
  Divider,
  TextField,
} from "@mui/material";
import { styled } from "@mui/material/styles";
import {
  AccountBox,
  DarkMode,
  Email,
  ManageAccounts,
  Money,
  Palette,
  Password,
  PlagiarismTwoTone,
} from "@mui/icons-material";

const headerHeight = 50;
const SettingContainer = styled(Box)(({ theme }) => ({
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
enum settingType {
  "switch",
  "text",
}
interface setting {
  name: string;
  description?: string;
  type: settingType;
  icon: JSX.Element;
}
interface settingGroup {
  title: string;
  description?: string;
  icon?: JSX.Element;
  settings: setting[];
}

export default function SettingsBrowser() {
  const settingList: settingGroup[] = [
    {
      title: "Account",
      description: "Your personal settings",
      icon: <ManageAccounts fontSize='large' />,
      settings: [
        {
          name: "Username",
          description: "Your name, that everyone will see.",
          type: settingType.text,
          icon: <AccountBox />,
        },
        {
          name: "Email address",
          description: "The email we use to contact you.",
          type: settingType.text,
          icon: <Email />,
        },
        {
          name: "Password",
          description: "Change your password",
          type: settingType.text,
          icon: <Password />,
        },
      ],
    },
    {
      title: "Appearance",
      description: "Change how Archytex looks",
      icon: <Palette fontSize='large' />,
      settings: [
        {
          name: "Dark mode",
          description: "Set the application theme to dark.",
          type: settingType.switch,
          icon: <DarkMode />,
        },
      ],
    },
    {
      title: "Billing & plans",
      description: "The services you get",
      icon: <Money />,
      settings: [
        {
          name: "Plan",
          description: "Select plan",
          type: settingType.switch,
          icon: <PlagiarismTwoTone />,
        },
      ],
    },
  ];
  const [checked, setChecked] = React.useState([""]);

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

  const getSettingComponent = (setting: setting) => {
    switch (setting.type) {
      case settingType.switch:
        return (
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
        );

      case settingType.text:
        return (
          <React.Fragment>
            <Box display='flex' justifyContent='start' flexWrap='wrap'>
              <ListItem
                key={setting.name}
                disablePadding
                sx={{ paddingX: 2, width: { xs: "100%", md: "50%" } }}
              >
                <ListItemIcon>{setting.icon}</ListItemIcon>
                <ListItemText
                  primary={setting.name}
                  secondary={setting.description}
                />
              </ListItem>
              <Box marginBottom={2} paddingLeft={2}>
                <TextField
                  size='small'
                  id={setting.name}
                  variant='outlined'
                  margin='none'
                />
              </Box>
            </Box>
          </React.Fragment>
        );
      default:
        break;
    }
  };
  //TODO: Use scrollspy
  // https://codesandbox.io/s/material-demo-xu80m?file=/ScrollSpyTabs.js
  return (
    <SettingContainer>
      {settingList.map((settingGroup: settingGroup, index) => (
        <Box
          key={settingGroup.title}
          marginBottom={2}
          marginTop={index === 0 ? 0 : 5}
        >
          <Box
            marginBottom={2}
            display='flex'
            justifyContent='start'
            paddingLeft={2}
          >
            <Box marginY='auto' marginTop={1.5} marginRight={4}>
              {settingGroup.icon}
            </Box>
            <Box>
              <Typography variant='h6'>{settingGroup.title}</Typography>
              <Typography variant='caption'>
                {settingGroup.description}
              </Typography>
            </Box>
          </Box>
          <Divider />
          <List>
            {settingGroup.settings.map((setting: setting) => (
              <React.Fragment>{getSettingComponent(setting)}</React.Fragment>
            ))}
          </List>
        </Box>
      ))}
    </SettingContainer>
  );
}
