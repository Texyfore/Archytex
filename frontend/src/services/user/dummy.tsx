import React, { useEffect, useState } from "react";
import { ApiContext, UserController } from "./api";


const DummyProvider = ({ children, fallback }: JSX.ElementChildrenAttribute & { fallback: JSX.Element }) => {
    const [value, setValue] = useState<UserController>(null);
    function getDefault(): UserController{
        return {
            state: "not-logged-in",
            logIn: async (username: string, password: string, _) => {
                setValue({
                    state: "logged-in",
                    user: {
                        username: username,
                        coins: 0,
                        email: `${username}@archytex.com`
                    }
                })
            },
            logOut: ()=>{
                setValue(getDefault())
            }
        }
    }
    useEffect(()=>{
        setTimeout(() => {
            setValue(getDefault());
        }, 1000);
    }, []);
    return value == null ? fallback : <ApiContext.Provider value={value}>
        {children}
    </ApiContext.Provider>
}

export { DummyProvider };