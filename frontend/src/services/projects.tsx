import React, { useContext, useEffect } from "react";
import houseImage4 from "../img/4.jpg";
import houseImage5 from "../img/5.jpg";
import houseImage6 from "../img/6.jpg";
import houseImage7 from "../img/7.jpg";
import houseImage8 from "../img/8.jpg";
import houseImage9 from "../img/9.jpg";
import houseImage10 from "../img/10.jpg";
import houseImage11 from "../img/11.jpg";
import houseImage12 from "../img/12.jpg";

interface DeleteProject {
  type: "delete-project";
  id: string;
}
interface RenameProject {
  type: "rename-project";
  id: string;
  name: string;
}
interface CreateProject {
  type: "create-project";
  name: string;
}

interface SetState {
  type: "set-state";
  state: ProjectsState;
}

type ProjectAction = DeleteProject | RenameProject | CreateProject | SetState;

interface Project {
  id: string;
  name: string;
  created: string;
  renders: Render[];
}

interface Render {
  id: string;
  renderName: string;
  status: number;
  renderTime: string;
  img: string;
}

interface ProjectsState {
  projects: Project[];
}

const ProjectsContext = React.createContext<{
  state: ProjectsState | undefined;
  dispatch: React.Dispatch<ProjectAction>;
}>({ state: undefined, dispatch: () => {} });

function useProjects() {
  return useContext(ProjectsContext);
}

function projectsReducer(
  state: ProjectsState | undefined,
  action: ProjectAction
): ProjectsState | undefined {
  //TODO: Send requests to server
  if (state === undefined) {
    state = { projects: [] };
  }
  switch (action.type) {
    case "delete-project":
      return {
        projects: state.projects.filter((p) => p.id !== action.id),
      };
    case "rename-project":
      return {
        projects: state.projects.map((p) =>
          p.id !== action.id
            ? p
            : {
                ...p,
                name: action.name,
              }
        ),
      };
    case "create-project":
      return {
        projects: [
          ...state.projects,
          {
            id: Math.round(Math.random() * 10000).toString(),
            created: new Date().toLocaleDateString("en-US"),
            name: action.name,
            renders: [
              {
                id: Math.round(Math.random() * 10000).toString(),
                renderName: action.name + "-render-1",
                status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
                renderTime: "1 h 40 min 23 sec",
                img: houseImage12,
              },
              {
                id: Math.round(Math.random() * 10000).toString(),
                renderName: action.name + "-render-1",
                status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
                renderTime: "2 h 30 min 23 sec",
                img: houseImage11,
              },
              {
                id: Math.round(Math.random() * 10000).toString(),
                renderName: action.name + "-render-1",
                status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
                renderTime: "20 min 23 sec",
                img: houseImage10,
              },
            ],
          },
        ],
      };
    case "set-state":
      return action.state;
  }
}

function ProjectsProvider({
  children,
}: {
  children: JSX.Element;
}): JSX.Element {
  //TODO: Load state from server
  const [state, dispatch]: [
    state: ProjectsState | undefined,
    dispatch: React.Dispatch<ProjectAction>
  ] = React.useReducer(projectsReducer, undefined);

  useEffect(() => {
    const initialstate: ProjectsState = {
      projects: [
        {
          id: Math.round(Math.random() * 10000).toString(),
          name: "Nice House",
          created: "Now",
          renders: [
            {
              id: Math.round(Math.random() * 10000).toString(),
              renderName:
                "Project1" +
                "-project-render-1-and it's very long so it can be abbreviated",
              status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
              renderTime: "1 h 40 min 23 sec",
              img: houseImage10,
            },
            {
              id: Math.round(Math.random() * 10000).toString(),
              renderName: "Project1-project-render-2",
              status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
              renderTime: "1000h 35 min 21 sec",
              img: houseImage9,
            },
            {
              id: Math.round(Math.random() * 10000).toString(),
              renderName: "Project1-project-render-2",
              status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
              renderTime: "1000h 35 min 21 sec",
              img: houseImage8,
            },
            {
              id: Math.round(Math.random() * 10000).toString(),
              renderName: "Project1-project-render-2",
              status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
              renderTime: "1000h 35 min 21 sec",
              img: houseImage7,
            },
          ],
        },
        {
          id: Math.round(Math.random() * 10000).toString(),
          name: "Test project 2",
          created: "Now",
          renders: [
            {
              id: Math.round(Math.random() * 10000).toString(),
              renderName:
                "Project2" +
                "-project-render-1-and it's very long so it can be abbreviated",
              status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
              renderTime: "1 h 40 min 23 sec",
              img: houseImage4,
            },
            {
              id: Math.round(Math.random() * 10000).toString(),
              renderName: "Project1-project-render-2",
              status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
              renderTime: "1000h 35 min 21 sec",
              img: houseImage6,
            },
            {
              id: Math.round(Math.random() * 10000).toString(),
              renderName: "Project1-project-render-2",
              status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
              renderTime: "1000h 35 min 21 sec",
              img: houseImage5,
            },
            {
              id: Math.round(Math.random() * 10000).toString(),
              renderName: "Project1-project-render-2",
              status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
              renderTime: "1000h 35 min 21 sec",
              img: houseImage7,
            },
          ],
        },
        {
          id: Math.round(Math.random() * 10000).toString(),
          name: "Test project 3",
          created: "Now",
          renders: [
            {
              id: Math.round(Math.random() * 10000).toString(),
              renderName:
                "Project2" +
                "-project-render-1-and it's very long so it can be abbreviated",
              status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
              renderTime: "1 h 40 min 23 sec",
              img: houseImage11,
            },
            {
              id: Math.round(Math.random() * 10000).toString(),
              renderName: "Project1-project-render-2",
              status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
              renderTime: "1000h 35 min 21 sec",
              img: houseImage6,
            },
            {
              id: Math.round(Math.random() * 10000).toString(),
              renderName: "Project1-project-render-2",
              status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
              renderTime: "1000h 35 min 21 sec",
              img: houseImage8,
            },
            {
              id: Math.round(Math.random() * 10000).toString(),
              renderName: "Project1-project-render-2",
              status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
              renderTime: "1000h 35 min 21 sec",
              img: houseImage9,
            },
            {
              id: Math.round(Math.random() * 10000).toString(),
              renderName: "Project1-project-render-2",
              status: Math.random() > 0.5 ? Math.random() * 100 : 100, //percentage
              renderTime: "1000h 35 min 21 sec",
              img: houseImage10,
            },
          ],
        },
      ],
    } as ProjectsState;
    const handle = setTimeout(() => {
      dispatch({
        type: "set-state",
        state: initialstate,
      });
    }, 1000);
    return () => {
      clearTimeout(handle);
    };
  }, [dispatch]);

  const value = { state, dispatch };
  return (
    <ProjectsContext.Provider value={value}>
      {children}
    </ProjectsContext.Provider>
  );
}

export { ProjectsProvider, ProjectsContext, useProjects };
export type { Project, Render };
