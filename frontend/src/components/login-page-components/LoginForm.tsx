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
  ["@media (max-height: 612px)"]: {
    height: "unset",
  },
}));

export default function LoginForm() {
  const [showPassword, setShowPassword] = useState(false);

  const handleClickShowPassword = () => {
    setShowPassword(!showPassword);
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
      sx={{
        backgroundColor: "background.paper",
      }}
    >
      <Box
        display='flex'
        flexDirection='column'
        justifyContent='center'
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
          width='100%'
          display='flex'
          alignItems='center'
          justifyContent='center'
          marginTop={3}
        >
          <Box
            height={1.01}
            sx={{ backgroundColor: "primary.main" }}
            width='100%'
          />
          <Typography variant='h6' fontWeight={600} fontSize='1em' paddingX={2}>
            LOGIN
          </Typography>
          <Box
            height={1.01}
            sx={{ backgroundColor: "primary.main" }}
            width='100%'
          />
        </Box>

        {/* Input form */}
        <Box
          display='flex'
          flexDirection='column'
          alignItems='center'
          paddingX={{ sm: 0, md: 6 }}
          marginBottom={1}
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
              label='Username'
              variant='standard'
              margin='normal'
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
              <InputLabel htmlFor='adornment-password'>Password</InputLabel>
              <Input
                required
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
              />
            </FormControl>
          </Box>

          <Box
            display='flex'
            justifyContent='start'
            width='304px'
            marginTop={2}
          >
            <FormControlLabel
              value='end'
              control={<Checkbox />}
              label={<Typography variant='caption'>Stay signed in</Typography>}
              labelPlacement='end'
            />
          </Box>
          <Button variant='outlined' sx={{ width: 304, marginY: 2 }}>
            Sign in
          </Button>
          <Typography variant='caption'>Don't have an account?</Typography>
          <Link variant='caption' href='#'>
            Sign up to Archytex
          </Link>
        </Box>
        {/* Use third-party */}
        <Box display='flex' flexDirection='column' alignItems='center'>
          <Box
            height='100%'
            display='flex'
            alignItems='center'
            justifyContent='center'
            width={304}
            marginY={2}
          >
            <Box
              height={1.01}
              sx={{ backgroundColor: "GrayText" }}
              width='100%'
            />
            <Typography
              variant='caption'
              fontWeight={600}
              paddingX={2}
              color='GrayText'
            >
              OR
            </Typography>
            <Box
              height={1.01}
              sx={{ backgroundColor: "GrayText" }}
              width='100%'
            />
          </Box>
          <Button variant='contained' sx={{ width: 304, marginY: 2 }}>
            Sign in with Google
          </Button>
          <Button
            variant='contained'
            sx={{
              width: 304,
              marginTop: 2,
              marginBottom: 3,
              backgroundColor: "#f5f0f6",
            }}
          >
            Sign in with Apple
          </Button>
        </Box>
      </Box>
    </MaxHeightContainer>
  );
}
