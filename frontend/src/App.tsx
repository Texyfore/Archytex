import React, { Suspense } from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import i18n from "i18next";
import { initReactI18next } from "react-i18next";
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
            <Typography marginTop={2}>Just a moment...</Typography>
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
            <Route path='/editor'>
              <Editor />
            </Route>
            <Route path='/editor'>
              <Editor />
            </Route>
            <Route path='/editor'>
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
