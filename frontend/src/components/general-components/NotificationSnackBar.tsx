import React from "react";

import Snackbar from "@mui/material/Snackbar";
import MuiAlert, { AlertProps } from "@mui/material/Alert";

import useNotification from "../../services/hooks/useNotification";

const Alert = React.forwardRef<HTMLDivElement, AlertProps>(function Alert(
  props,
  ref
) {
  return <MuiAlert elevation={6} ref={ref} variant='filled' {...props} />;
});

export default function NotificationSnackBar() {
  const { notification, removeNotification } = useNotification();

  const handleClose = (
    event?: React.SyntheticEvent | Event,
    reason?: string
  ) => {
    if (reason === "clickaway") {
      return;
    }

    removeNotification();
  };

  return (
    <Snackbar
      anchorOrigin={{ vertical: "bottom", horizontal: "center" }}
      open={notification !== null}
      autoHideDuration={5000}
      onClose={handleClose}
    >
      <Alert
        onClose={handleClose}
        severity={notification?.status}
        sx={{ width: "100%" }}
      >
        {notification?.message}
      </Alert>
    </Snackbar>
  );
}
