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
interface CreateProject {
    type: "create-project",
    name: string
}

type ProjectAction = DeleteProject | RenameProject | CreateProject;

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
}

interface ProjectsState {
    projects: Project[]
}

const ProjectsContext = React.createContext(null as unknown as { state: ProjectsState, dispatch: React.Dispatch<ProjectAction> });

function useProjects() {
    return useContext(ProjectsContext);
}

function projectsReducer(state: ProjectsState, action: ProjectAction): ProjectsState {
    //TODO: Send requests to server
    switch (action.type) {
        case "delete-project":
            return { 
                projects: state.projects.filter(p=>p.id != action.id)
             };
        case "rename-project":
            return { ...state };
        case "create-project":
            return {
                projects: [
                    ...state.projects,
                    {
                        id: Date.now().toString(),
                        created: new Date().toLocaleDateString("en-US"),
                        name: action.name,
                        renders: [
                            {
                                id: Date.now().toString(),
                                renderName:
                                    "Project1" +
                                    "-project-render-1-and it's very long so it can be abbreviated",
                                status: Math.random()*100, //percentage
                                renderTime: "1 h 40 min 23 sec",
                            },
                            {
                                id: (Date.now()+1).toString(),
                                renderName: "Project1" + "-project-render-2",
                                status: Math.random()*100, //percentage
                                renderTime: "1000h 35 min 21 sec",
                            },
                        ]
                    }
                ]
            };
    }
}

function ProjectsProvider({ children }: { children: JSX.Element }): JSX.Element {
    //TODO: Load state from server
    const initialstate: ProjectsState = {
        projects: [
            {
                id: Date.now().toString(),
                name: "Project1",
                created: "Now",
                renders: [
                    {
                        id: Date.now().toString(),
                        renderName:
                            "Project1" +
                            "-project-render-1-and it's very long so it can be abbreviated",
                        status: Math.random()*100, //percentage
                        renderTime: "1 h 40 min 23 sec",
                    },
                    {
                        id: (Date.now()+1).toString(),
                        renderName: "Project1" + "-project-render-2",
                        status: Math.random()*100, //percentage
                        renderTime: "1000h 35 min 21 sec",
                    },
                ]
            }
        ]
    } as ProjectsState;
    const [state, dispatch] = React.useReducer(projectsReducer, initialstate);
    const value = { state, dispatch };
    return <ProjectsContext.Provider value={value}>{children}</ProjectsContext.Provider>
}

export { ProjectsProvider, ProjectsContext, useProjects}
export type {Project}