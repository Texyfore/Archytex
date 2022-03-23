import React, { useEffect, useState } from "react";
import { ApiContext, Callback, User, UserController } from "./api";
import Environment from "../../env";
import { TypeOfTag } from "typescript";
import {
  Project,
  ProjectsDispatch,
  Subscription,
  Render,
  Action,
} from "../projects";
import internal from "stream";

const USER_URL = `${Environment.base_url}auth/user`;
const LOGIN_URL = `${Environment.base_url}login`;

function get_fetch(token: string) {
  return async function authenticatedFetch(
    resource: RequestInfo,
    init?: RequestInit
  ) {
    let headers: HeadersInit = {
      Authorization: "Bearer " + token,
      ...init?.headers,
    };
    let _init: RequestInit = {
      ...init,
      headers,
    };
    let result;
    result = await fetch(resource, _init);
    if (result.status !== 200) {
      let data = await result.json();
      throw { message: data.error, requestId: data._requestId };
    }
    return result;
  };
}

type Internal = {
  user: User;
  fetch: (
    resource: RequestInfo,
    init?: RequestInit | undefined
  ) => Promise<Response>;
  token: string;
} | null;

async function Restore(token: string | null): Promise<Internal> {
  if (token === null) {
    token = localStorage.getItem("token");
    if (token == null) {
      return null;
    }
  }
  let fetch = get_fetch(token);
  let response = await fetch(USER_URL, { method: "POST" });
  let user: User = await response.json();
  return { user, fetch, token };
}

async function LogIn(
  username: string,
  password: string,
  stayLoggedIn: Boolean
) {
  var resp = await fetch(LOGIN_URL, {
    headers: {
      "Content-Type": "application/json",
    },
    method: "POST",
    body: JSON.stringify({
      username,
      password,
    }),
  });
  let data = await resp.json();
  if (resp.status !== 200) {
    throw { message: data.error, requestId: data._requestId };
  }
  if (stayLoggedIn) {
    localStorage.setItem("token", data.token);
  }
  return await Restore(data.token);
}

interface RenderUpdate {
  id: string;
  name: string;
  status: number;
  started: string;
  finished: string;
  icon: string;
}

interface ProjectUpdate {
  id: string;
  title: string;
  created: string;
  renders: RenderUpdate[];
}

function convertRender(r: RenderUpdate): Render {
  return {
    ...r,
    started: new Date(r.started),
    finished: new Date(r.finished),
  };
}

function convertProjectUpdate(p: ProjectUpdate): Project {
  return {
    ...p,
    created: new Date(p.created),
    renders: p.renders.map(convertRender),
  };
}

interface Updates {
  projects: ProjectUpdate[] | undefined;
}

const PROJECT_URL = `${Environment.base_url}auth/project`;

const subscribe: (internal: Internal) => (callback: Callback) => {
  dispatch: ProjectsDispatch;
  dispose: () => void;
} = (internal: Internal) => (callback: Callback) => {
  const ws = new WebSocket(Environment.ws_url);
  ws.addEventListener("open", () => {
    ws.send(JSON.stringify(internal?.token));
  });
  ws.addEventListener("message", (ev: MessageEvent<string>) => {
    const data: Updates = JSON.parse(ev.data);
    const converted = data.projects?.map(convertProjectUpdate);
    callback(converted ?? []);
  });
  return {
    dispose: () => {
      ws.close();
    },
    dispatch: async (action: Action) => {
      switch (action.type) {
        case "create":
          await internal?.fetch(PROJECT_URL, {
            headers: {
              "Content-Type": "application/json",
            },
            method: "POST",
            body: JSON.stringify(action.name),
          });
          return;
        case "delete":
          await internal?.fetch(PROJECT_URL + "/" + action.id, {
            headers: {
              "Content-Type": "application/json",
            },
            method: "DELETE",
          });
          return;
        case "rename":
          await internal?.fetch(PROJECT_URL + "/" + action.id, {
            headers: {
              "Content-Type": "application/json",
            },
            method: "PATCH",
            body: JSON.stringify(action.name),
          });
          return;
        case "deleteRender":
          await internal?.fetch(`${PROJECT_URL}/${action.id}/render/${action.render}`, {
            headers: {
              "Content-Type": "application/json",
            },
            method: "DELETE",
          });
          return;
      }
    },
  };
};

const RestProvider = ({
  children,
  fallback,
}: JSX.ElementChildrenAttribute & { fallback: JSX.Element }) => {
  const [value, setValue] = useState<UserController>(null);
  const [internal, setInternal] = useState<Internal | undefined>(undefined);
  useEffect(() => {
    Restore(null).then(setInternal);
  }, []);
  useEffect(() => {
    if (internal === undefined) {
      setValue(null);
      return;
    }
    if (internal === null) {
      setValue({
        state: "not-logged-in",
        logIn: async (username, password, stayLoggedIn) => {
          const data = await LogIn(username, password, stayLoggedIn);
          setInternal(data);
        },
      });
      return;
    }
    setValue({
      state: "logged-in",
      user: internal.user,
      logOut: () => {
        setInternal(null);
        localStorage.removeItem("token");
      },
      subscribe: subscribe(internal),
      save: async (data: Uint8Array, id: string) => {
        internal.fetch(`${Environment.base_url}auth/project/${id}/data`, {
          method: "POST",
          body: data,
          headers: { "Content-Type": "application/octet-stream" },
        });
      },
      render: async (data: Uint8Array, id: string, width: number, height: number, samples: number) => {
        internal.fetch(`${Environment.base_url}auth/project/${id}/render/${width}/${height}/${samples}`, {
          method: "POST",
          body: data,
          headers: { "Content-Type": "application/octet-stream" },
        });
      },
      load: async (id: string) => {
        const result = await internal.fetch(
          `${Environment.base_url}auth/project/${id}/data`,
          {
            method: "GET",
          }
        );
        return new Uint8Array(await result.arrayBuffer());
      },
      modifyUser: async(data) => {
        await internal.fetch(
          USER_URL,
          {
            method: "PATCH",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify(data)
          },
          
        );
      }
    });
  }, [internal]);
  return value == null ? (
    fallback
  ) : (
    <ApiContext.Provider value={value}>{children}</ApiContext.Provider>
  );
};

export { RestProvider };
