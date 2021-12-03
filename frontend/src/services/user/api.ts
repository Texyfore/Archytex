import React, { useContext } from "react";

interface User {
    username: string
}

interface UserLoggedIn {
    state: "logged-in"
    user: User
}
interface LoginResult {
    //TODO: LoginResult
}
interface UserNotLoggedIn {
    state: "not-logged-in"
    logIn: (username: string, password: string) => LoginResult
}

type UserController = UserLoggedIn | UserNotLoggedIn | null;


const ApiContext = React.createContext<UserController>(null)

const useApi = ()=>{
    return useContext(ApiContext);
}

export type { User, UserController }
export { ApiContext, useApi }