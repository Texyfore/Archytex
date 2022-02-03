import React, { useState } from "react";
import {
  Box,
  Button,
  FormControl,
  FormHelperText,
  IconButton,
  Input,
  InputAdornment,
  InputLabel,
  Link,
  TextField,
  Typography,
} from "@mui/material";
import { styled } from "@mui/material/styles";
import {
  AccountCircle,
  Apple,
  Google,
  Mail,
  RedoRounded,
  Visibility,
  VisibilityOff,
  VpnKey,
} from "@mui/icons-material";
import houseImage12 from "../../img/12.jpg";
import ColoredReCaptcha from "../ColoredReCaptcha";
import { Register } from "../../services/register";
import { useHistory } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { Link as L } from "react-router-dom";

const MaxHeightContainer = styled(Box)(({ theme }) => ({
  backgroundColor: "background.paper",
  marginTop: 56,
  height: `calc(100vh - 56px)`,
  [`${theme.breakpoints.up("xs")} and (orientation: landscape)`]: {
    marginTop: 48,
    height: "unset",
  },
  [theme.breakpoints.up("sm")]: {
    marginTop: 64,
    height: "unset",
  },
  [theme.breakpoints.up("md")]: {
    marginTop: 64,
    height: `calc(100vh - 64px)`,
  },
  // eslint-disable-next-line no-useless-computed-key
  ["@media (max-height: 750px)"]: {
    height: "unset",
  },
}));
export default function RegisterForm() {
  const { t } = useTranslation();

  const [showPassword, setShowPassword] = useState(false);
  const [showRePassword, setShowRePassword] = useState(false);
  const [captchaKey, setCaptchaKey] = useState(0);

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

  const [captcha, setCaptcha] = useState<string | null>(null);
  const history = useHistory();

  const register = () => {
    //TODO: Translate errors

    let errored = false;
    if (username === "") {
      setUsernameError("Username can't be empty");
      errored = true;
    }
    if (email === "") {
      setEmailError("Email can't be empty");
      errored = true;
    }
    if (!email.includes("@")) {
      setEmailError("Invalid email format");
      errored = true;
    }
    if (password === "") {
      setPasswordError("Password can't be empty");
      errored = true;
    }
    if (password !== rePassword) {
      setRePasswordError("The two passwords don't match");
      errored = true;
    }
    if (rePassword === "") {
      setRePasswordError("Please write the password again");
      errored = true;
    }
    if (captcha === null) {
      setGeneralError("Please complete the reCAPTCHA");
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
          setGeneralError("reCAPTCHA check failed");
          return;
        });
    }
  };

  const handleClickShowPassword = () => {
    setShowPassword(!showPassword);
  };
  const handleClickShowRePassword = () => {
    setShowRePassword(!showRePassword);
  };
  const handleMouseDownPassword = (
    event: React.MouseEvent<HTMLButtonElement>
  ) => {
    event.preventDefault();
  };

  return (
    <MaxHeightContainer
      display='flex'
      justifyContent='center'
      alignItems='center'
      sx={{ flexDirection: { xs: "column", md: "row" } }}
    >
      <Box
        display='flex'
        flexDirection='column'
        justifyContent='center'
        width={{ xs: "100%", md: "405.333px" }}
        height={{ xs: "300px", md: "650px" }}
        borderRadius={2}
        sx={{
          backgroundColor: "background.paper",
          filter: "drop-shadow(0px 0px 4px rgba(0,0,0,0.25))",
        }}
      >
        {/* Image */}
        <Box
          height='50%'
          display={{ xs: "none", md: "flex" }}
          sx={{
            backgroundImage: `url(${houseImage12})`,
            backgroundSize: "cover",
          }}
        ></Box>
        {/* Use third-party */}
        <Box
          paddingX={{ sm: 0, md: 6 }}
          height='50%'
          display='flex'
          flexDirection='column'
          justifyContent='center'
          alignItems='center'
        >
          <Typography
            variant='h3'
            fontSize={24}
            fontWeight={200}
            paddingX={2}
            marginBottom={4}
            textAlign='center'
          >
            {t("get_started_with_archytex")}
          </Typography>
          <Button
            variant='contained'
            sx={{ width: 304, marginY: 2, backgroundColor: "#f5f0f6" }}
            endIcon={<Google color='primary' fontSize='large' />}
          >
            <Typography variant='button' color='primary.main'>
              {t("sign_in_with_google")}
            </Typography>
          </Button>
          <Button
            variant='contained'
            sx={{
              width: 304,
              marginTop: 2,
              marginBottom: 3,
              backgroundColor: "#f5f0f6",
            }}
            endIcon={<Apple fontSize='large' />}
          >
            <Typography variant='button'>{t("sign_in_with_apple")}</Typography>
          </Button>
        </Box>
      </Box>
      <Box
        display='flex'
        flexDirection='column'
        justifyContent='space-around'
        width={{ xs: "100%", md: "unset" }}
        height={{ xs: "100%", md: "650px" }}
        borderRadius={2}
        sx={{
          backgroundColor: "background.paper",
          filter: "drop-shadow(0px 0px 4px rgba(0,0,0,0.25))",
        }}
      >
        {/* Register title */}
        <Box
          width='304px'
          marginX='auto'
          display='flex'
          alignItems='center'
          justifyContent='center'
          marginTop={3}
        >
          <Box
            flexGrow={1}
            height={1.01}
            sx={{ backgroundColor: "primary.main" }}
          />
          <Typography variant='h6' fontWeight={600} fontSize='1em' paddingX={2}>
            {t("register").toUpperCase()}
          </Typography>
          <Box
            flexGrow={1}
            height={1.01}
            sx={{ backgroundColor: "primary.main" }}
          />
        </Box>

        {/* Input form */}
        <Box
          display='flex'
          flexDirection='column'
          alignItems='center'
          paddingX={{ sm: 0, md: 6 }}
          marginBottom={1}
          maxHeight='400px'
        >
          <Box
            sx={{ display: "flex", alignItems: "flex-end" }}
            display='flex'
            alignItems='flex-end'
            width='304px'
          >
            <AccountCircle sx={{ mr: 1, my: 1 }} />
            <TextField
              id='standard-required'
              helperText={usernameError}
              error={usernameError !== ""}
              label={t("username")}
              variant='standard'
              margin='normal'
              type='text'
              value={username}
              onChange={(e) => handleUsernameChange(e)}
            />
          </Box>
          <Box
            sx={{ display: "flex", alignItems: "flex-end" }}
            display='flex'
            alignItems='flex-end'
            width='304px'
          >
            <Mail sx={{ mr: 1, my: 1 }} />
            <TextField
              id='standard-required'
              helperText={emailError}
              error={emailError !== ""}
              label={t("email")}
              variant='standard'
              margin='normal'
              type='email'
              value={email}
              onChange={(e) => handleEmailChange(e)}
            />
          </Box>
          <Box
            display='flex'
            alignItems='flex-end'
            width='304px'
            marginTop={2}
            marginBottom={1}
          >
            <VpnKey sx={{ mr: 1, my: 1 }} />

            <FormControl sx={{ width: "304px" }} variant='standard'>
              <InputLabel htmlFor='adornment-password'>
                <Typography color={passwordError !== "" ? "error" : "info"}>
                  {t("password")}
                </Typography>
              </InputLabel>
              <Input
                required
                onChange={(e) => handlePasswordChange(e)}
                error={passwordError !== ""}
                id='adornment-password'
                type={showPassword ? "text" : "password"}
                endAdornment={
                  <InputAdornment position='end'>
                    <IconButton
                      aria-label='toggle password visibility'
                      onClick={handleClickShowPassword}
                      onMouseDown={handleMouseDownPassword}
                      edge='end'
                      sx={{ marginRight: "0.1px" }}
                    >
                      {showPassword ? <VisibilityOff /> : <Visibility />}
                    </IconButton>
                  </InputAdornment>
                }
                value={password}
              />
              <FormHelperText>
                <Typography
                  variant='caption'
                  color={passwordError !== "" ? "error" : "info"}
                >
                  {passwordError}
                </Typography>
              </FormHelperText>
            </FormControl>
          </Box>
          <Box
            display='flex'
            alignItems='flex-end'
            width='304px'
            marginTop={2}
            marginBottom={1}
          >
            <RedoRounded sx={{ mr: 1, my: 1 }} />

            <FormControl sx={{ width: "304px" }} variant='standard'>
              <InputLabel htmlFor='adornment-repassword'>
                <Typography color={rePasswordError !== "" ? "error" : "info"}>
                  {t("password_again")}
                </Typography>
              </InputLabel>
              <Input
                required
                onChange={(e) => handleRePasswordChange(e)}
                error={rePasswordError !== ""}
                id='adornment-repassword'
                type={showRePassword ? "text" : "password"}
                endAdornment={
                  <InputAdornment position='end'>
                    <IconButton
                      aria-label='toggle password visibility'
                      onClick={handleClickShowRePassword}
                      onMouseDown={handleMouseDownPassword}
                      edge='end'
                      sx={{ marginRight: "0.1px" }}
                    >
                      {showRePassword ? <VisibilityOff /> : <Visibility />}
                    </IconButton>
                  </InputAdornment>
                }
                value={rePassword}
              />
              <FormHelperText>
                <Typography
                  variant='caption'
                  color={rePasswordError !== "" ? "error" : "info"}
                >
                  {rePasswordError}
                </Typography>
              </FormHelperText>
            </FormControl>
          </Box>
        </Box>

        {/* ReCAPTCHA */}
        <Box paddingY={1} display='flex' justifyContent='center'>
          <ColoredReCaptcha
            sitekey='6Lc5gWodAAAAAEVg3MPTn5Nj7KN-ishnafqV4ZL8'
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
      </Box>
    </MaxHeightContainer>
  );
}
