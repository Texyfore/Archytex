import React, { useEffect, useState } from "react";
import { ApiContext, UserController } from "./api";


const DummyProvider = ({ children, fallback }: JSX.ElementChildrenAttribute & { fallback: JSX.Element }) => {
    const [value, setValue] = useState<UserController>(null);
    useEffect(()=>{
        setTimeout(() => {
            setValue({
                state: "not-logged-in",
                logIn: (username, password) => {
                    setValue({
                        state: "logged-in",
                        user: {
                            username: username
                        }
                    })
                    return {};
                }
            });
        }, 1000);
    }, []);
    return value == null ? fallback : <ApiContext.Provider value={value}>
        {children}
    </ApiContext.Provider>
}

export { DummyProvider };