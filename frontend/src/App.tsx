import React, { Suspense } from "react";

import { BrowserRouter as Router, Switch, Route } from "react-router-dom";

import CssBaseline from "@mui/material/CssBaseline";

import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import translationEn from "./languages/en_us.json";
import translationHu from "./languages/hu_hu.json";
import translationJp from "./languages/jp_jp.json";
import translationUa from "./languages/uk.json";
import translationSe from "./languages/sv.json";
import translationRo from "./languages/ro.json";
import translationDe from "./languages/de.json";
import translationFr from "./languages/fr.json";
import translationBg from "./languages/bg.json";

import { DummyProvider } from "./services/user/dummy";
import { RestProvider } from "./services/user/rest";
import { ColorModeProvider } from "./services/colorMode";
import NotificationProvider from "./services/notification";

import ArchytexAppBar from "./components/app-bar-components/ArchytexAppBar";
import SuspenseFallback from "./components/general-components/SuspenseFallback";
import ScrollToTop from "./components/general-components/ScrollToTop";
import NotificationSnackBar from "./components/general-components/NotificationSnackBar";

import Home from "./pages/Home";
import Dashboard from "./pages/Dashboard";
import Settings from "./pages/Settings";
import Login from "./pages/Login";
import Register from "./pages/Register";
import About from "./pages/About";
import Features from "./pages/Features";
import Editor from "./pages/Editor";
import PageNotFound from "./pages/PageNotFound";
import SuccessfulRegistration from "./pages/SuccessfulRegistration";

i18n.use(initReactI18next).init({
  resources: {
    en: { translation: translationEn },
    hu: { translation: translationHu },
    jp: { translation: translationJp },
    ua: { translation: translationUa },
    se: { translation: translationSe },
    ro: { translation: translationRo },
    de: { translation: translationDe },
    fr: { translation: translationFr },
    bg: { translation: translationBg },
  },
  lng: "en",
  fallbackLng: "en",
  interpolation: { escapeValue: false },
});

function App() {
  return (
    <Suspense fallback={<SuspenseFallback />}>
      <CssBaseline />
      <RestProvider fallback={<SuspenseFallback />}>
        <NotificationProvider>
          <Router>
            <ArchytexAppBar />

            <ScrollToTop />
            <Switch>
              <Route exact path='/'>
                <Home />
              </Route>
              <Route exact path='/login'>
                <Login />
              </Route>
              <Route exact path='/register'>
                <Register />
              </Route>
              <Route exact path='/about'>
                <About />
              </Route>
              <Route exact path='/features'>
                <Features />
              </Route>
              <Route exact path='/dashboard'>
                <Dashboard />
              </Route>
              <Route exact path='/settings'>
                <Settings />
              </Route>
              <Route exact path='/success'>
                <SuccessfulRegistration />
              </Route>
              <Route path='/editor/:projectId'>
                <Editor />
              </Route>
              <Route>
                <PageNotFound />
              </Route>
            </Switch>
          </Router>
          <NotificationSnackBar />
        </NotificationProvider>
      </RestProvider>
    </Suspense>
  );
}

export default function ToggleColorMode() {
  return (
    <ColorModeProvider>
      <App />
    </ColorModeProvider>
  );
}
