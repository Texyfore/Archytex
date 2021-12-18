import React, { useEffect, useState } from "react";
import { ApiContext, User, UserController } from "./api";
import Environment from "../../env";
import { TypeOfTag } from "typescript";

const USER_URL = `${Environment.base_url}auth/user`;
const LOGIN_URL = `${Environment.base_url}login`;

function get_fetch(token: string){
    return function authenticatedFetch(resource: RequestInfo, init?: RequestInit) {
        let headers:HeadersInit = {
            "Authorization": "Bearer " + token,
            ...init?.headers
        };
        let _init: RequestInit = {
            headers,
            ...init
        }
        return fetch(resource, _init)
    };
}

type Internal = { user: User; fetch: (resource: RequestInfo, init?: RequestInit | undefined) => Promise<Response>; token: string; } | null ;

async function Restore(token: string | null): Promise<Internal>{
    if (token === null) {
        token = localStorage.getItem("token");
        if (token == null) {
            return null;
        }
    }
    let fetch = get_fetch(token);
    let response = await fetch(USER_URL, {method: "POST"});
    if (response.status === 200) {
        let user: User = await response.json();
        return {user, fetch, token};
    }else{
        return null;
    }
}

async function LogIn(username: string, password: string, stayLoggedIn: Boolean){
    var resp = await fetch(LOGIN_URL, {
        headers: {
            "Content-Type": "application/json",
        },
        method: "POST",
        body: JSON.stringify({
            username, password
        })
    });
    let data = await resp.json();
    if (resp.status !== 200) {
        throw {message: data.message, requestId: data._requestId}
    }
    if (stayLoggedIn) {
        localStorage.setItem("token", data.token);
    }
    return await Restore(data.token)
}


const RestProvider = ({ children, fallback }: JSX.ElementChildrenAttribute & { fallback: JSX.Element }) => {
    const [value, setValue] = useState<UserController>(null);
    const [internal, setInternal] = useState<Internal|undefined>(undefined);
    useEffect(()=>{
        Restore(null).then(setInternal)
    }, []);
    useEffect(()=>{
        if (internal === undefined) {
            setValue(null);
            return;
        }
        if (internal === null) {
            setValue({
                state: "not-logged-in",
                logIn: async (username, password, stayLoggedIn)=>{
                    const data = await LogIn(username, password, stayLoggedIn);
                    setInternal(data);
                },
                logOut: ()=>setInternal(null)
            })
            return;
        }
        setValue({
            state: "logged-in",
            user: internal.user
        });
    }, [internal]);
    return value == null ? fallback : <ApiContext.Provider value={value}>
        {children}
    </ApiContext.Provider>
}

export { RestProvider };