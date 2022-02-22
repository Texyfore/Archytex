import React, { useContext } from "react";

import { NotificationContext } from "../notification";

function useNotification() {
  const { notification, addNotification, removeNotification } =
    useContext(NotificationContext);
  return { notification, addNotification, removeNotification };
}

export default useNotification;
