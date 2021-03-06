import React, { useContext, useEffect } from "react";
import { useHistory } from "react-router-dom";
import { Projects, ProjectsDispatch } from "../projects";

interface User {
  username: string;
  email: string;
  coins: number;
}
type Callback = (projects: Projects) => void;

interface ModifyUserType{
  username: string | undefined,
  password: string | undefined,
  email: string | undefined
}

interface UserLoggedIn {
  state: "logged-in";
  logOut: () => void;
  user: User;
  subscribe: (callback: Callback) => {
    dispatch: ProjectsDispatch;
    dispose: () => void;
  };
  save: (data: Uint8Array, id: string) => Promise<void>;
  load: (id: string) => Promise<Uint8Array | undefined>;
  render: (data: Uint8Array, id: string, width: number, height: number, samples: number) => Promise<void>;
  modifyUser: (data: ModifyUserType) => Promise<void>;
}
interface UserNotLoggedIn {
  state: "not-logged-in";
  logIn: (
    username: string,
    password: string,
    stayLoggedIn: boolean
  ) => Promise<void>;
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

export type { User, UserController, Callback, ModifyUserType };
export { ApiContext, useApi };
