import React, { useState } from "react";

import ProjectBrowserHeader from "./ProjectBrowserHeader";
import ProjectList from "./ProjectList";

import { ProjectsProvider } from "../../../services/projects";

export default function ProjectBrowser() {
  const [query, setQuery] = useState("");
  const handleQueryChange = (query: string) => {
    setQuery(query);
    console.log(query);
  };
  return (
    <ProjectsProvider>
      <ProjectBrowserHeader
        query={query}
        handleQueryChange={handleQueryChange}
      />
      <ProjectList query={query} />
    </ProjectsProvider>
  );
}
