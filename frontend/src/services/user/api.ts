import React, { useContext, useEffect } from "react";
import { useHistory } from "react-router-dom";

interface User {
  username: string;
  email: string;
  coins: number;
}

interface UserLoggedIn {
  state: "logged-in";
  user: User;
}
interface UserNotLoggedIn {
  state: "not-logged-in";
  logIn: (
    username: string,
    password: string,
    stayLoggedIn: boolean
  ) => Promise<void>;
  logOut: () => void;
}

type UserController = UserLoggedIn | UserNotLoggedIn | null;

const ApiContext = React.createContext<UserController>(null);

const useApi = (required?: boolean) => {
  const val = useContext(ApiContext);
  const history = useHistory();
  useEffect(() => {
    if (required && val?.state === "not-logged-in") {
      history.push("/login");
    }
  }, [val]);
  return val;
};

export type { User, UserController };
export { ApiContext, useApi };
