import React, { Suspense } from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";

import { DummyProvider } from "./services/user/dummy";
import { RestProvider } from "./services/user/rest";

import CssBaseline from "@mui/material/CssBaseline";

import { ColorModeProvider } from "./services/colorMode";

import i18n from "i18next";
import { initReactI18next, useTranslation } from "react-i18next";
import translationEn from "./languages/en_us.json";
import translationHu from "./languages/hu_hu.json";
import translationJp from "./languages/jp_jp.json";

import Home from "./pages/Home";
import Login from "./pages/Login";
import Register from "./pages/Register";
import About from "./pages/About";
import PageNotFound from "./pages/PageNotFound";

import SuspenseFallback from "./components/general-components/SuspenseFallback";
import ArchytexAppBar from "./components/app-bar-components/ArchytexAppBar";

i18n.use(initReactI18next).init({
  resources: {
    en: { translation: translationEn },
    hu: { translation: translationHu },
    jp: { translation: translationJp },
  },
  lng: "en",
  fallbackLng: "en",
  interpolation: { escapeValue: false },
});

function App() {
  const { t } = useTranslation();
  return (
    <Suspense fallback={<SuspenseFallback />}>
      <CssBaseline />
      <ArchytexAppBar />
      <DummyProvider fallback={<SuspenseFallback />}>
        <Router>
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
            {/* <Route path='/dashboard'>
              <Dashboard />
            </Route>
            <Route path='/login'>
              <LoginPage />
            </Route>
            <Route path='/register'>
              <RegisterPage />
            </Route>
            <Route path='/editor/:projectId'>
              <Editor />
            </Route>
            <Route path='/success'>
              <SuccessfulRegistration />
            </Route>
             */}
            <Route>
              <PageNotFound />
            </Route>
          </Switch>
        </Router>
      </DummyProvider>
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
