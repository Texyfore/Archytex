import React, { useState } from "react";

import Box from "@mui/material/Box";

import Typography from "@mui/material/Typography";
import IconButton from "@mui/material/IconButton";
import FormControl from "@mui/material/FormControl";
import FormHelperText from "@mui/material/FormHelperText";
import Input from "@mui/material/Input";
import InputLabel from "@mui/material/InputLabel";
import InputAdornment from "@mui/material/InputAdornment";

import {
  AccountCircle,
  Mail,
  RedoRounded,
  Visibility,
  VisibilityOff,
  VpnKey,
} from "@mui/icons-material";

type Variant =
  | "regular"
  | "username"
  | "password"
  | "email"
  | "repeatPassword"
  | "number";

interface Props {
  variant: Variant;
  label: string;
  input: string | number;
  inputChange: (e: any) => void;
  error: string;
}

export default function FormInput({
  variant,
  label,
  input,
  inputChange,
  error,
}: Props) {
  const [showPassword, setShowPassword] = useState(false);
  const handleClickShowPassword = () => {
    setShowPassword(!showPassword);
  };
  const handleMouseDownPassword = (
    event: React.MouseEvent<HTMLButtonElement>
  ) => {
    event.preventDefault();
  };

  const getIcon = (variant: Variant) => {
    switch (variant) {
      case "username":
        return <AccountCircle sx={{ mr: 1, my: 1 }} />;
      case "email":
        return <Mail sx={{ mr: 1, my: 1 }} />;
      case "password":
        return <VpnKey sx={{ mr: 1, my: 1 }} />;
      case "repeatPassword":
        return <RedoRounded sx={{ mr: 1, my: 1 }} />;
      default:
        return null;
    }
  };

  return (
    <Box
      display='flex'
      alignItems='flex-end'
      width={variant === "regular" || variant === "number" ? "100%" : "304px"}
      marginTop={2}
      marginBottom={1}
    >
      {getIcon(variant)}
      <FormControl
        sx={{
          width:
            variant === "regular" || variant === "number" ? "100%" : "304px",
        }}
        variant='standard'
      >
        {variant === "number" ? (
          <InputLabel shrink>
            <Typography color='info'>{label}</Typography>
          </InputLabel>
        ) : (
          <InputLabel htmlFor='adornment-password'>
            <Typography color={error !== "" ? "error" : "info"}>
              {label}
            </Typography>
          </InputLabel>
        )}
        <Input
          error={error !== ""}
          required
          id='adornment-password'
          type={
            variant === "password" || variant === "repeatPassword"
              ? showPassword
                ? "text"
                : "password"
              : variant === "number"
              ? "number"
              : "text"
          }
          endAdornment={
            variant === "password" || variant === "repeatPassword" ? (
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
            ) : (
              <></>
            )
          }
          value={input}
          onChange={inputChange}
        />

        <FormHelperText>
          <Typography variant='caption' color={error !== "" ? "error" : "info"}>
            {error}
          </Typography>
        </FormHelperText>
      </FormControl>
    </Box>
  );
}
