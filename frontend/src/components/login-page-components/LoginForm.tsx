import React, { useState } from "react";
import {
  Box,
  Button,
  Checkbox,
  FormControl,
  FormControlLabel,
  IconButton,
  Input,
  InputAdornment,
  InputLabel,
  Link,
  TextField,
  Typography,
} from "@mui/material";
import {
  AccountCircle,
  Visibility,
  VisibilityOff,
  VpnKey,
} from "@mui/icons-material";
import { styled } from "@mui/material/styles";
import { useApi } from "../../services/user/api";
import { useHistory } from "react-router";
import { useTranslation } from "react-i18next";

const MaxHeightContainer = styled(Box)(({ theme }) => ({
  marginTop: 56,
  height: `calc(100vh - 56px)`,
  [`${theme.breakpoints.up("xs")} and (orientation: landscape)`]: {
    marginTop: 48,
    height: `calc(100vh - 48px)`,
  },
  [theme.breakpoints.up("sm")]: {
    marginTop: 64,
    height: `calc(100vh - 64px)`,
  },
  // eslint-disable-next-line no-useless-computed-key
  ["@media (max-height: 700px)"]: {
    height: "unset",
  },
}));

export default function LoginForm() {
  const { t } = useTranslation();

  const [showPassword, setShowPassword] = useState(false);

  const handleClickShowPassword = () => {
    setShowPassword(!showPassword);
  };

  const handleMouseDownPassword = (
    event: React.MouseEvent<HTMLButtonElement>
  ) => {
    event.preventDefault();
  };

  const api = useApi();

  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [stayLoggedIn, setStayLoggedIn] = useState(false);
  const history = useHistory();
  const loginClick = () => {
    if (api?.state === "not-logged-in") {
      //TODO: Handle login result
      api.logIn(username, password, stayLoggedIn).then(() => {
        history.push("/dashboard");
      });
    }
  };

  return (
    <MaxHeightContainer
      display="flex"
      justifyContent="center"
      alignItems="center"
      sx={{
        backgroundColor: "background.paper",
      }}
    >
      <Box
        display="flex"
        flexDirection="column"
        justifyContent="center"
        width={{ xs: "100%", md: "unset" }}
        height={{ xs: "100%", md: "unset" }}
        borderRadius={2}
        sx={{
          backgroundColor: "background.paper",
          filter: "drop-shadow(0px 0px 4px rgba(0,0,0,0.25))",
        }}
      >
        {/* Login title */}
        <Box
          width="304px"
          marginX="auto"
          display="flex"
          alignItems="center"
          justifyContent="center"
          marginTop={3}
        >
          <Box
            height={1.01}
            sx={{ backgroundColor: "primary.main" }}
            width="100%"
          />
          <Typography variant="h6" fontWeight={600} fontSize="1em" paddingX={2}>
            {t("login").toUpperCase()}
          </Typography>
          <Box
            height={1.01}
            sx={{ backgroundColor: "primary.main" }}
            width="100%"
          />
        </Box>

        {/* Input form */}
        <Box
          display="flex"
          flexDirection="column"
          alignItems="center"
          paddingX={{ sm: 0, md: 6 }}
          marginBottom={1}
        >
          <Box
            sx={{ display: "flex", alignItems: "flex-end" }}
            display="flex"
            alignItems="flex-end"
            width="304px"
          >
            <AccountCircle sx={{ mr: 1, my: 1 }} />
            <TextField
              id="standard-required"
              label={t("username")}
              variant="standard"
              margin="normal"
              value={username}
              onChange={(ev) => setUsername(ev.target.value)}
            />
          </Box>
          <Box
            display="flex"
            alignItems="flex-end"
            width="304px"
            marginTop={2}
            marginBottom={1}
          >
            <VpnKey sx={{ mr: 1, my: 1 }} />

            <FormControl sx={{ width: "304px" }} variant="standard">
              <InputLabel htmlFor="adornment-password">
                {t("password")}
              </InputLabel>
              <Input
                required
                id="adornment-password"
                type={showPassword ? "text" : "password"}
                endAdornment={
                  <InputAdornment position="end">
                    <IconButton
                      aria-label="toggle password visibility"
                      onClick={handleClickShowPassword}
                      onMouseDown={handleMouseDownPassword}
                      edge="end"
                      sx={{ marginRight: "0.1px" }}
                    >
                      {showPassword ? <VisibilityOff /> : <Visibility />}
                    </IconButton>
                  </InputAdornment>
                }
                value={password}
                onChange={(ev) => setPassword(ev.target.value)}
              />
            </FormControl>
          </Box>

          <Box
            display="flex"
            justifyContent="start"
            width="304px"
            marginTop={2}
          >
            <FormControlLabel
              value="end"
              control={
                <Checkbox
                  checked={stayLoggedIn}
                  onChange={(ev) => setStayLoggedIn(ev.target.checked)}
                />
              }
              label={
                <Typography variant="caption">{t("stay_signed_in")}</Typography>
              }
              labelPlacement="end"
            />
          </Box>
          <Button
            variant="outlined"
            sx={{ width: 304, marginY: 2 }}
            onClick={loginClick}
          >
            {t("sign_in")}
          </Button>
          <Typography variant="caption">{t("dont_have_an_account")}</Typography>
          <Link variant="caption" href="/register">
            {t("sign_up_to_archytex")}
          </Link>
        </Box>
        {/* Use third-party */}
        <Box display="flex" flexDirection="column" alignItems="center">
          <Box
            height="100%"
            display="flex"
            alignItems="center"
            justifyContent="center"
            width={304}
            marginY={2}
          >
            <Box
              height={1.01}
              sx={{ backgroundColor: "GrayText" }}
              width="100%"
            />
            <Typography
              variant="caption"
              fontWeight={600}
              paddingX={2}
              color="GrayText"
            >
              {t("or").toUpperCase()}
            </Typography>
            <Box
              height={1.01}
              sx={{ backgroundColor: "GrayText" }}
              width="100%"
            />
          </Box>
          <Button variant="contained" sx={{ width: 304, marginY: 2 }}>
            {t("sign_in_with_google")}
          </Button>
          <Button
            variant="contained"
            sx={{
              width: 304,
              marginTop: 2,
              marginBottom: 3,
              backgroundColor: "#f5f0f6",
            }}
          >
            {t("sign_in_with_apple")}
          </Button>
        </Box>
      </Box>
    </MaxHeightContainer>
  );
}
