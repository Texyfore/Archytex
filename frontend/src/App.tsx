import React, { Suspense, useMemo } from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import i18n from "i18next";
import { initReactI18next, useTranslation } from "react-i18next";
import "./App.css";
import { ThemeProvider, createTheme } from "@mui/material/styles";
import CssBaseline from "@mui/material/CssBaseline";
import MainPage from "./pages/MainPage";
import Dashboard from "./pages/Dashboard";
import LoginPage from "./pages/LoginPage";
import RegisterPage from "./pages/RegisterPage";
import {
  ColorMode,
  ColorModeProvider,
  modeToString,
  useColorMode,
} from "./services/colorMode";
import { CircularProgress } from "@mui/material";
import { DummyProvider } from "./services/user/dummy";

//TODO: Get translations from api
const translationEn = {
  archytex: "Archytex",
  motto: "Take your architectural visualisations to the next level",
  learn_more: "Learn More",
  what_is_archytex: "What is Archytex?",
  p1: "Archytex is a lightweight 3D architecture design tool, combined with fast and powerful ray-traced rendering. All while staying in your web browser.",
  archviz_in_your_browser: "Archviz in your browser",
  start_creating_now: "Start creating now",
  blazing_fast_rendering: "Blazing fast rendering",
  try_it_out:
    "Try our server side ray-traced rendering solution, now with a 1 month free trial",
  subscribe_now: "Subscribe Now",
};
const translationHu = {
  archytex: "Archytex",
  motto: "Emelje magasabb szitre építészeti látványterveit",
  learn_more: "Tudj meg többet",
  what_is_archytex: "Mi az Archytex?",
  p1: "Az Archytex egy alacsony hardverigényű 3 dimenziós építészeti design eszköz, egy gyors és erős sugárkövetéses renderelőmotorral összefűzve. Mindez végig a böngészőben.",
  archviz_in_your_browser: "Építészeti látványterv a böngészőben",
  start_creating_now: "Kezdés most",
  blazing_fast_rendering: "Hihetetlenül gyors renderelés",
  try_it_out:
    "Próbálja ki a szerver-oldali sugárkövetéses renderelő motorunkat, most 1 hónapos ingyenes próbaidőszakkal",
  subscribe_now: "Feliratkozás most",
};
const translationJp = {
  archytex: "アーキテックス",
  motto: "建築ビジュアライゼーションを次のレベルへ",
  learn_more: "さらに詳しく",
  what_is_archytex: "アーキテックスとは何ですか？",
  p1: "アーキテックスは、軽量な3D建築デザインツールであり、高速でパワフルなレイトレースレンダリングを兼ね備えています。しかも、ブラウザ上で動作します。",
  archviz_in_your_browser: "あなたのブラウザでアーキビズ",
  start_creating_now: "今すぐ作成を開始",
  blazing_fast_rendering: "超高速なレンダリング",
  try_it_out:
    "今、サーバーサイドレイトレースレンダリングソリューションを1ヶ月間無料でお試しいただけます。",
  subscribe_now: "今すぐ登録",
};

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
  //set different font for Japanese language
  const { i18n } = useTranslation();
  const fontFamily = useMemo(() => {
    switch (i18n.language) {
      case "jp":
        return "Noto Sans JP";

      default:
        return "Poppins";
    }
  }, [i18n.language]);
  const [mode] = useColorMode();
  const archytex_theme = React.useMemo(
    () =>
      createTheme({
        palette: {
          mode: modeToString(mode),
          primary: {
            main: "#39A0ED",
          },
          secondary: {
            main: "#f68dd1",
          },
          text: {
            primary: mode === ColorMode.Dark ? "#f5f0f6" : "#0c0c0c",
          },
          background: {
            default: mode === ColorMode.Dark ? "#0c0c0c" : "#F5F0F6",
            paper: mode === ColorMode.Dark ? "#0c0c0c" : "#F5F0F6",
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
          divider: mode === ColorMode.Dark ? "#1F1F1F" : "#EBE7EC",
        },
        shape: {
          borderRadius: 2,
        },
        typography: {
          fontFamily: fontFamily,
        },
      }),
    [mode, fontFamily]
  );

  return (
    <ThemeProvider theme={archytex_theme}>
      <Suspense fallback={<CircularProgress color='primary' />}>
        <CssBaseline />
        <DummyProvider fallback={<CircularProgress color='primary' />}>
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
        </DummyProvider>
      </Suspense>
    </ThemeProvider>
  );
}

export default function ToggleColorMode() {
  return (
    <ColorModeProvider>
      <App />
    </ColorModeProvider>
  );
}
