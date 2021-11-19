import React from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import "./App.css";
import { ThemeProvider } from "@mui/material/styles";
import { createTheme } from "@mui/material/styles";
import CssBaseline from "@mui/material/CssBaseline";
import MainPage from "./pages/MainPage";
import Dashboard from "./pages/Dashboard";
import LoginPage from "./pages/LoginPage";
import RegisterPage from "./pages/RegisterPage";

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
  );
}

export default App;
