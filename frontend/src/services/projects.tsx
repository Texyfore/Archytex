import React from "react";
import { useContext } from "react";
import { useApi } from "./user/api";

interface Render{
    id: string
    name: string
    status: number,
    started: Date,
    finished: Date,
    icon: string
}

interface Project{
    id: string
    title: string
    created: Date
    renders: Render[]
}

interface ActionCreate{
    type: "create"
    name: string
}

interface ActionRename{
    type: "rename"
    id: string
    name: string
}

interface ActionDelete{
    type: "delete"
    id: string
}

type Projects = Project[];
type Action = ActionCreate | ActionRename | ActionDelete;
type ProjectsDispatch = (action: Action)=>void
type Subscription = { projects: Projects, dispatch: ProjectsDispatch, dispose: ()=>void};

const ProjectsContext = React.createContext<Subscription>(undefined as unknown as Subscription)

const ProjectsProvider = ({children}: JSX.ElementChildrenAttribute)=>{
    const api = useApi(true);
    if (api?.state === "not-logged-in") {
        return null;
    }
    const sub = api?.subscribe();
    return <ProjectsContext.Provider value={sub as Subscription}>
        {children}
    </ProjectsContext.Provider>
}

const useProjects = ()=>{
    return useContext(ProjectsContext);
}

export {useProjects, ProjectsProvider}

export type {Project, Projects, Render,Action, ActionCreate,ActionRename, ActionDelete, ProjectsDispatch, Subscription}