import * as React from "react";

import { useTranslation } from "react-i18next";

import SwipeableViews from "react-swipeable-views";

import { useTheme } from "@mui/material/styles";

import AppBar from "@mui/material/AppBar";
import Tabs from "@mui/material/Tabs";
import Tab from "@mui/material/Tab";
import Box from "@mui/material/Box";

import { useSubPage } from "../../services/selectedDashboardSubPage";

import ProjectBrowser from "./projects-subpage-components/ProjectBrowser";
import SettingsBrowser from "./settings-subpage-components/SettingBrowser";

interface TabPanelProps {
  children?: React.ReactNode;
  dir?: string;
  index: number;
  value: number;
}

function TabPanel(props: TabPanelProps) {
  const { children, value, index, ...other } = props;

  return (
    <div
      role='tabpanel'
      hidden={value !== index}
      id={`full-width-tabpanel-${index}`}
      aria-labelledby={`full-width-tab-${index}`}
      {...other}
    >
      {value === index && <Box>{children}</Box>}
    </div>
  );
}

function a11yProps(index: number) {
  return {
    id: `full-width-tab-${index}`,
    "aria-controls": `full-width-tabpanel-${index}`,
  };
}

export default function MobileDashboard() {
  const { t } = useTranslation();

  const theme = useTheme();

  const [subPage, setSubPage] = useSubPage();

  const [value, setValue] = React.useState(0);
  React.useEffect(() => {
    setValue(subPage === "projects" ? 0 : 1);
  }, [subPage]);

  const handleChange = (event: React.SyntheticEvent, newValue: number) => {
    setSubPage(newValue === 0 ? "projects" : "settings");
    setValue(newValue);
  };

  const handleChangeIndex = (index: number) => {
    setValue(index);
  };

  return (
    <Box bgcolor='background.paper' width='100%' overflow='hidden'>
      <AppBar position='static' color='inherit' elevation={0}>
        <Tabs
          value={value}
          onChange={handleChange}
          indicatorColor='primary'
          textColor='inherit'
          variant='fullWidth'
        >
          <Tab label={t("projects")} {...a11yProps(0)} />
          <Tab label={t("settings")} {...a11yProps(1)} />
        </Tabs>
      </AppBar>
      <SwipeableViews
        axis={theme.direction === "rtl" ? "x-reverse" : "x"}
        index={value}
        onChangeIndex={handleChangeIndex}
        containerStyle={{ height: "100%" }}
      >
        <TabPanel value={value} index={0} dir={theme.direction}>
          <ProjectBrowser />
        </TabPanel>
        <TabPanel value={value} index={1} dir={theme.direction}>
          <SettingsBrowser />
        </TabPanel>
      </SwipeableViews>
    </Box>
  );
}
