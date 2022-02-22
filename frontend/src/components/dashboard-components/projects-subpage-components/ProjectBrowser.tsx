import React from "react";

import ProjectBrowserHeader from "./ProjectBrowserHeader";
import ProjectList from "./ProjectList";

import { ProjectsProvider } from "../../../services/projects";

export default function ProjectBrowser() {
  return (
    <ProjectsProvider>
      <ProjectBrowserHeader />
      <ProjectList />
    </ProjectsProvider>
  );
}
