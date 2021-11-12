import React from "react"

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

interface ProjectsState {

}

const ProjectsContext = React.createContext(null as unknown as { state: ProjectsState, dispatch: React.Dispatch<ProjectAction> });

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
    const [state, dispatch] = React.useReducer(projectsReducer, {});
    const value = {state, dispatch};
    return <ProjectsContext.Provider value={value}></ProjectsContext.Provider>
}

export { ProjectsProvider, ProjectsContext }