import React, { useContext, useEffect } from "react";
import { useHistory } from "react-router-dom";
import { Projects, ProjectsDispatch } from "../projects";

interface User {
  username: string;
  email: string;
  coins: number;
}
type Callback = (projects: Projects)=>void;
interface UserLoggedIn {
    state: "logged-in"
    logOut: ()=>void
    user: User
    subscribe: ( callback: Callback )=>{dispatch: ProjectsDispatch, dispose: ()=>void}
}
interface UserNotLoggedIn {
    state: "not-logged-in"
    logIn: (username: string, password: string, stayLoggedIn: boolean) => Promise<void>

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

export type { User, UserController, Callback }
export { ApiContext, useApi }