import React, { useState } from "react";

import { useHistory, Link as L } from "react-router-dom";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import Checkbox from "@mui/material/Checkbox";
import FormControlLabel from "@mui/material/FormControlLabel";
import Link from "@mui/material/Link";

import FormContainer from "../form-components/FormContainer";
import FormInput from "../form-components/FormInput";

import { useApi } from "../../services/user/api";
import useNotification from "../../services/hooks/useNotification";

type ErrorType = "username" | "password" | "general";

export default function LoginForm() {
  const { t } = useTranslation();

  const api = useApi();

  const history = useHistory();

  const { addNotification } = useNotification();

  const [username, setUsername] = useState("");
  const [usernameError, setUsernameError] = useState("");
  const handleUsernameChange = (e: any) => {
    eraseErrors();
    setUsername(e.target.value);
  };

  const [password, setPassword] = useState("");
  const [passwordError, setPasswordError] = useState("");
  const handlePasswordChange = (e: any) => {
    eraseErrors();
    setPassword(e.target.value);
  };

  const [generalError, setGeneralError] = useState("");

  const eraseErrors = () => {
    setUsernameError("");
    setPasswordError("");
    setGeneralError("");
  };

  const [stayLoggedIn, setStayLoggedIn] = useState(false);

  const loginClick = () => {
    if (username === "") {
      handleError(t("empty_username"), "username");
      if (password !== "") {
        return;
      }
    }
    if (password === "") {
      handleError(t("empty_password"), "password");
      return;
    }

    if (api?.state === "not-logged-in") {
      api
        .logIn(username, password, stayLoggedIn)
        .then(() => {
          history.push("/dashboard");
          addNotification(t("successful_login"), "success");
        })
        .catch((error) => {
          handleError(error.message, "general");
          return;
        });
    }
  };

  const handleError = (errorMessage: string, errorType: ErrorType) => {
    switch (errorType) {
      case "username":
        setUsernameError(errorMessage);
        break;
      case "password":
        setPasswordError(errorMessage);
        break;
      case "general":
        setGeneralError(errorMessage);
        break;

      default:
        setGeneralError(errorMessage);
        break;
    }
  };

  return (
    <FormContainer title={t("login").toUpperCase()}>
      {/* Username */}
      <FormInput
        variant='username'
        label={t("username")}
        input={username}
        inputChange={handleUsernameChange}
        error={usernameError}
      />
      {/* Password */}
      <FormInput
        variant='password'
        label={t("password")}
        input={password}
        inputChange={handlePasswordChange}
        error={passwordError}
      />
      {/* General error */}
      <Box marginTop={2}>
        <Typography color='error' variant='body2'>
          {generalError}
        </Typography>
      </Box>
      {/* Submit */}
      <Button
        variant='outlined'
        sx={{ width: 304, marginY: 2 }}
        onClick={loginClick}
      >
        {t("sign_in")}
      </Button>
      <Typography variant='caption'>{t("dont_have_an_account")}</Typography>
      <Link variant='caption' component={L} to='/register'>
        {t("sign_up_to_archytex")}
      </Link>
    </FormContainer>
  );
}
