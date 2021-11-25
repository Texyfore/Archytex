import React, { useState } from "react";
import {
  Box,
  Button,
  FormControl,
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
  Mail,
  RedoRounded,
  Visibility,
  VisibilityOff,
  VpnKey,
} from "@mui/icons-material";
import houseImage12 from "../../img/12.jpg";

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
  const [showPassword, setShowPassword] = useState(false);
  const [showRePassword, setShowRePassword] = useState(false);

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
        height={{ xs: "300px", md: "565px" }}
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
            Get started with Archytex
          </Typography>
          <Button
            variant='contained'
            sx={{ width: 304, marginY: 2, backgroundColor: "#f5f0f6" }}
          >
            <Typography variant='button' color='primary.main'>
              Sign in with Google
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
          >
            <Typography variant='button'>Sign in with Apple</Typography>
          </Button>
        </Box>
      </Box>
      <Box
        display='flex'
        flexDirection='column'
        justifyContent='space-around'
        width={{ xs: "100%", md: "unset" }}
        height={{ xs: "100%", md: "565px" }}
        borderRadius={2}
        sx={{
          backgroundColor: "background.paper",
          filter: "drop-shadow(0px 0px 4px rgba(0,0,0,0.25))",
        }}
      >
        {/* Register title */}
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
            REGISTER
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
              type='text'
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
              label='Email'
              variant='standard'
              margin='normal'
              type='email'
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
            alignItems='flex-end'
            width='304px'
            marginTop={2}
            marginBottom={1}
          >
            <RedoRounded sx={{ mr: 1, my: 1 }} />

            <FormControl sx={{ width: "304px" }} variant='standard'>
              <InputLabel htmlFor='adornment-repassword'>
                Password again
              </InputLabel>
              <Input
                required
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
              />
            </FormControl>
          </Box>
        </Box>

        {/* Submit button */}
        <Box
          display='flex'
          flexDirection='column'
          alignItems='center'
          paddingX={{ sm: 0, md: 6 }}
          marginBottom={1}
        >
          <Button variant='outlined' sx={{ width: 304, marginY: 2 }}>
            Sign up
          </Button>
          <Typography variant='caption'>Already have an account?</Typography>
          <Link variant='caption' href='#'>
            Log in to Archytex
          </Link>
        </Box>
      </Box>
    </MaxHeightContainer>
  );
}
