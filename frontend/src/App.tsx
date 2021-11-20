import React, { Suspense } from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import "./App.css";
import { ThemeProvider } from "@mui/material/styles";
import { createTheme } from "@mui/material/styles";
import CssBaseline from "@mui/material/CssBaseline";
import MainPage from "./pages/MainPage";
import Dashboard from "./pages/Dashboard";
import LoginPage from "./pages/LoginPage";
import RegisterPage from "./pages/RegisterPage";

//TODO: Get translations from api
const translationEn = {
  motto: "Take your architectural visualisations to the next level",
};
const translationHu = {
  motto: "Emelje magasabb szitre építészeti látványterveit",
};
const translationJp = {};

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
  const archytex_theme = createTheme({
    palette: {
      mode: "dark",
      primary: {
        main: "#39A0ED",
      },
      secondary: {
        main: "#f68dd1",
      },
      text: {
        primary: "#f5f0f6",
      },
      background: {
        default: "#0c0c0c",
        paper: "#0c0c0c",
      },
      error: {
        main: "#fb4d3d",
      },
      warning: {
        main: "#fea82f",
      },
      info: {
        main: "#4c6085",
      },
      success: {
        main: "#13c4a3",
      },
      divider: "#f5f0f6",
    },
    shape: {
      borderRadius: 1,
    },
    typography: {
      fontFamily: "Poppins",
    },
  });

  return (
    <Suspense fallback='Loading...'>
      <ThemeProvider theme={archytex_theme}>
        <CssBaseline />
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
          </Switch>
        </Router>
      </ThemeProvider>
    </Suspense>
  );
}

export default App;
