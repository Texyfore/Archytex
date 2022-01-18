import React, { Suspense } from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import i18n from "i18next";
import { initReactI18next, useTranslation } from "react-i18next";
import "./App.css";
import CssBaseline from "@mui/material/CssBaseline";
import MainPage from "./pages/MainPage";
import Dashboard from "./pages/Dashboard";
import LoginPage from "./pages/LoginPage";
import RegisterPage from "./pages/RegisterPage";
import { ColorModeProvider } from "./services/colorMode";
import { Box, CircularProgress, Typography } from "@mui/material";
import { DummyProvider } from "./services/user/dummy";
import Editor from "./pages/Editor";
import { RestProvider } from "./services/user/rest";
import PageNotFound from "./pages/PageNotFound";
import SuccessfulRegistration from "./pages/SuccessfulRegistration";
import translationEn from "./languages/en_us.json";
import translationHu from "./languages/hu_hu.json";
import translationJp from "./languages/jp_jp.json";

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
    <Suspense fallback={<CircularProgress color='primary' />}>
      <CssBaseline />
      <DummyProvider
        fallback={
          <Box
            display='flex'
            height='100vh'
            justifyContent='center'
            alignItems='center'
            flexDirection='column'
          >
            <CircularProgress color='primary' />
            <Typography marginTop={2}>{t("just_a_moment")}</Typography>
          </Box>
        }
      >
        <Router>
          <Switch>
            <Route exact path='/'>
              <MainPage />
            </Route>
            <Route path='/dashboard'>
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
