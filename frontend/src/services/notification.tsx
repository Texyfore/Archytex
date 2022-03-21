import { AlertColor } from "@mui/material";
import React, { useState, useCallback } from "react";

interface NotificationProps {
  message: undefined | string;
  status: undefined | AlertColor;
}
interface NotificationContextProps {
  notification: null | NotificationProps;
  addNotification: (
    message: undefined | string,
    status: undefined | AlertColor
  ) => void;
  removeNotification: () => void;
  open: boolean;
}
export const NotificationContext =
  React.createContext<NotificationContextProps>({
    notification: null,
    addNotification: (
      message: undefined | string,
      status: undefined | AlertColor
    ) => {},
    removeNotification: () => {},
    open: false,
  });

interface Props {
  children: undefined | JSX.Element | JSX.Element[];
}
export default function NotificationProvider({ children }: Props) {
  const [notification, setNotification] = useState<NotificationProps | null>(
    null
  );
  const [open, setOpen] = useState(false);

  const removeNotification = () => setOpen(false);

  const addNotification = (
    message: undefined | string,
    status: undefined | AlertColor
  ) => {
    setNotification({ message, status });
    setOpen(true);
  };

  const contextValue = {
    notification: notification,
    addNotification: useCallback(
      (message, status) => addNotification(message, status),
      []
    ),
    removeNotification: useCallback(() => removeNotification(), []),
    open: open,
  };

  return (
    <NotificationContext.Provider value={contextValue}>
      {children}
    </NotificationContext.Provider>
  );
}
