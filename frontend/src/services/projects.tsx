import React, { useContext } from "react"

interface DeleteProject {
    type: "delete-project";
    id: string;
}
interface RenameProject {
    type: "rename-project";
    id: string;
    name: string;
}

type ProjectAction = DeleteProject | RenameProject;

interface Project {
    name: string;
    created: string;
    renders: Render[];
}

interface Render {
    renderName: string;
    status: number;
    renderTime: string;
}

interface ProjectsState {
    projects: Project[]
}

const ProjectsContext = React.createContext(null as unknown as { state: ProjectsState, dispatch: React.Dispatch<ProjectAction> });

function useProjects() {
    return useContext(ProjectsContext);
}

function projectsReducer(state: ProjectsState, action: ProjectAction) {
    //TODO: Send requests to server
    switch (action.type) {
        case "delete-project":
            return { ...state };
        case "rename-project":
            return { ...state };
    }
}

function ProjectsProvider({ children }: { children: JSX.Element }): JSX.Element {
    //TODO: Load state from server
    const initialstate: ProjectsState = {
        projects: [
            {
                name: "Project1",
                created: "Now",
                renders: [
                    {
                        renderName:
                            "Project1" +
                            "-project-render-1-and it's very long so it can be abbreviated",
                        status: 74, //percentage
                        renderTime: "1 h 40 min 23 sec",
                    },
                    {
                        renderName: "Project1" + "-project-render-2",
                        status: 14, //percentage
                        renderTime: "1000h 35 min 21 sec",
                    },
                ],
            }
        ]
    } as ProjectsState;
    const [state, dispatch] = React.useReducer(projectsReducer, initialstate);
    const value = { state, dispatch };
    return <ProjectsContext.Provider value={value}>{children}</ProjectsContext.Provider>
}

export { ProjectsProvider, ProjectsContext, useProjects }