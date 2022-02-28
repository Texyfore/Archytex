import React, { useState } from "react";

import Box from "@mui/material/Box";

import ProjectBrowserHeader from "./ProjectBrowserHeader";
import ProjectList from "./ProjectList";

import { ProjectsProvider } from "../../services/projects";

export default function ProjectBrowser() {
  const [query, setQuery] = useState("");
  const handleQueryChange = (query: string) => {
    setQuery(query);
    console.log(query);
  };
  return (
    <ProjectsProvider>
      <Box width='100%'>
        <ProjectBrowserHeader
          query={query}
          handleQueryChange={handleQueryChange}
        />
        <ProjectList query={query} />
      </Box>
    </ProjectsProvider>
  );
}
