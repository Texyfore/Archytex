import React, { useState } from "react";
import { useEffect } from "react";
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
type ProjectsDispatch = (action: Action)=>Promise<void>
type Subscription = { projects: Projects, dispatch: ProjectsDispatch};



const ProjectsContext = React.createContext<Subscription>(undefined as unknown as Subscription)

const ProjectsProvider = ({children}: JSX.ElementChildrenAttribute)=>{
    const api = useApi(true);

    const [projects, setProjects] = useState([] as Projects)
    const [dispatch, setDispatch] = useState<ProjectsDispatch>(async ()=>{})
    useEffect(()=>{
        if (api?.state === "not-logged-in" || api === null) {
            return ()=>{};
        }
        const {dispatch, dispose} = api.subscribe((p)=>{
            setProjects(p);
        });
        setDispatch(()=>dispatch);
        return dispose;
    }, [api]);
    return <ProjectsContext.Provider value={{projects: projects, dispatch: dispatch}}>
        {children}
    </ProjectsContext.Provider>
}

const useProjects = ()=>{
    return useContext(ProjectsContext);
}

export {useProjects, ProjectsProvider}

export type {Project, Projects, Render,Action, ActionCreate,ActionRename, ActionDelete, ProjectsDispatch, Subscription}