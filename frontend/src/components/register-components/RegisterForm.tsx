import React, { useState } from "react";

import { useHistory, Link as L } from "react-router-dom";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import Link from "@mui/material/Link";

import FormContainer from "../form-components/FormContainer";
import FormInput from "../form-components/FormInput";
import ColoredReCaptcha from "../form-components/ColoredReCaptcha";

import { Register } from "../../services/register";
import Environment from "../../env";

export default function RegisterForm() {
  const { t } = useTranslation();

  const history = useHistory();

  const [captchaKey, setCaptchaKey] = useState(0);
  const [captcha, setCaptcha] = useState<string | null>(null);

  const [username, setUsername] = useState("");
  const [usernameError, setUsernameError] = useState("");
  const handleUsernameChange = (e: any) => {
    eraseErrors();
    setUsername(e.target.value);
  };

  const [email, setEmail] = useState("");
  const [emailError, setEmailError] = useState("");
  const handleEmailChange = (e: any) => {
    eraseErrors();
    setEmail(e.target.value);
  };

  const [password, setPassword] = useState("");
  const [passwordError, setPasswordError] = useState("");
  const handlePasswordChange = (e: any) => {
    eraseErrors();
    setPassword(e.target.value);
  };

  const [rePassword, setRePassword] = useState("");
  const [rePasswordError, setRePasswordError] = useState("");
  const handleRePasswordChange = (e: any) => {
    eraseErrors();
    setRePassword(e.target.value);
  };

  const [generalError, setGeneralError] = useState("");

  const eraseErrors = () => {
    setUsernameError("");
    setEmailError("");
    setPasswordError("");
    setRePasswordError("");
    setGeneralError("");
  };

  const register = () => {
    let errored = false;
    if (username === "") {
      setUsernameError(t("empty_username"));
      errored = true;
    }
    if (email === "") {
      setEmailError(t("empty_email"));
      errored = true;
    }
    if (!email.includes("@")) {
      setEmailError(t("invalid_email"));
      errored = true;
    }
    if (password === "") {
      setPasswordError(t("empty_password"));
      errored = true;
    }
    if (password !== rePassword) {
      setRePasswordError(t("passwords_dont_match"));
      errored = true;
    }
    if (rePassword === "") {
      setRePasswordError(t("write_password_again"));
      errored = true;
    }
    if (captcha === null) {
      setGeneralError(t("complete_recaptcha"));
      errored = true;
    }
    if (captcha !== null && !errored) {
      Register(username, password, email, captcha)
        .then(() => {
          history.push("/success");
        })
        .catch((err) => {
          alert(JSON.stringify(err));
          setCaptchaKey(captchaKey ^ 1);
          setGeneralError(t("recaptcha_failed"));
          return;
        });
    }
  };

  return (
    <FormContainer title={t("register").toUpperCase()}>
      {/* Username */}
      <FormInput
        variant={"username"}
        label={t("username")}
        input={username}
        inputChange={handleUsernameChange}
        error={usernameError}
      />
      {/* Email */}
      <FormInput
        variant={"email"}
        label={t("email")}
        input={email}
        inputChange={handleEmailChange}
        error={emailError}
      />
      {/* Password */}
      <FormInput
        variant={"password"}
        label={t("password")}
        input={password}
        inputChange={handlePasswordChange}
        error={passwordError}
      />
      {/* Password again */}
      <FormInput
        variant={"repeatPassword"}
        label={t("password_again")}
        input={rePassword}
        inputChange={handleRePasswordChange}
        error={rePasswordError}
      />

      {/* General error */}
      <Box marginTop={2}>
        <Typography color='error' variant='body2'>
          {generalError}
        </Typography>
      </Box>

      {/* ReCAPTCHA */}
      <Box paddingY={1} display='flex' justifyContent='center'>
        <ColoredReCaptcha
          sitekey={Environment.captcha}
          onChange={setCaptcha}
          key={captchaKey}
        />
      </Box>

      {/* Submit button */}
      <Box
        display='flex'
        flexDirection='column'
        alignItems='center'
        paddingX={{ sm: 0, md: 6 }}
        marginBottom={1}
      >
        <Button
          variant='outlined'
          sx={{ width: 304, marginY: 2 }}
          onClick={register}
        >
          {t("sign_up")}
        </Button>
        <Typography variant='caption'>
          {t("already_have_an_account")}
        </Typography>
        <Link variant='caption' component={L} to='/login'>
          {t("log_in_to_archytex")}
        </Link>
      </Box>
    </FormContainer>
  );
}
