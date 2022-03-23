import React, { useContext } from "react";

import { NotificationContext } from "../notification";

function useNotification() {
  const { notification, addNotification, removeNotification, open } =
    useContext(NotificationContext);
  return { notification, addNotification, removeNotification, open };
}

export default useNotification;
