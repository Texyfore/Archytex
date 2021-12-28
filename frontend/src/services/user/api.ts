import React, { useContext, useEffect } from "react";
import { useHistory } from "react-router-dom";
import { Projects, ProjectsDispatch, Subscription } from "../projects";

interface User {
    username: string,
    email: string,
    coins: number,
}

interface UserLoggedIn {
    state: "logged-in"
    logOut: ()=>void
    user: User
    subscribe: ()=>Subscription
}
interface UserNotLoggedIn {
    state: "not-logged-in"
    logIn: (username: string, password: string, stayLoggedIn: boolean) => Promise<void>
    
}

type UserController = UserLoggedIn | UserNotLoggedIn | null;


const ApiContext = React.createContext<UserController>(null)

const useApi = (required?: boolean)=>{
    const val = useContext(ApiContext);
    const history = useHistory();
    useEffect(()=>{
        if (required && val?.state == "not-logged-in") {
            history.push("/login")
        }
    }, [val]);
    return val;
}

export type { User, UserController }
export { ApiContext, useApi }