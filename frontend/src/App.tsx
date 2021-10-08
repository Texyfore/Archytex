import React from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import "./App.css";
import { ThemeProvider } from "@mui/material/styles";
import { createTheme } from "@mui/material/styles";
import MainPage from "./pages/MainPage";
import Dashboard from "./pages/Dashboard";
import LoginPage from "./pages/LoginPage";

function App() {
  const custom_theme = createTheme({});

  return (
    <ThemeProvider theme={custom_theme}>
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
        </Switch>
      </Router>
    </ThemeProvider>
  );
}

export default App;
