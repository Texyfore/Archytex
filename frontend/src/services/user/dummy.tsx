import { randomInt } from "crypto";
import React, { useEffect, useReducer, useState } from "react";
import { Action, Projects, ProjectsDispatch } from "../projects";
import { ApiContext, Callback, UserController } from "./api";

function reducer(projects: Projects, action: Action): Projects {
  switch (action.type) {
    case "create":
      return [
        ...projects,
        {
          title: action.name,
          created: new Date(),
          renders: [
            {
              name: "Render1",
              finished: new Date(),
              started: new Date(),
              id: Math.random().toString(),
              status: 0.5,
              icon: "/img/4.png",
            },
          ],
          id: Math.random().toString(),
        },
      ];
    case "delete":
      return projects.filter((p) => p.id !== action.id);
    case "rename":
      return projects.map((p) =>
        p.id == action.id
          ? {
              ...p,
              title: action.name,
            }
          : p
      );
  }
}

function Subscribe(callback: Callback): {
  dispatch: ProjectsDispatch;
  dispose: () => void;
} {
  let s: Projects = [];
  return {
    dispatch: async (action: Action) => {
      s = reducer(s, action);
      callback(s);
    },
    dispose: () => {},
  };
}

const DummyProvider = ({
  children,
  fallback,
}: JSX.ElementChildrenAttribute & { fallback: JSX.Element }) => {
  const [value, setValue] = useState<UserController>(null);
  function getDefault(): UserController {
    return {
      state: "not-logged-in",
      logIn: async (username: string, password: string, _) => {
        setValue({
          state: "logged-in",
          user: {
            username: username,
            coins: 0,
            email: `${username}@archytex.com`,
          },
          subscribe: Subscribe,
          logOut: () => {
            setValue(getDefault());
          },
          save: async (data: Uint8Array) => {
            console.log(data.length);
          },
        });
      },
    };
  }
  useEffect(() => {
    setTimeout(() => {
      setValue(getDefault());
    }, 1000);
  }, []);
  return value == null ? (
    fallback
  ) : (
    <ApiContext.Provider value={value}>{children}</ApiContext.Provider>
  );
};

export { DummyProvider };
