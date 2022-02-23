import React from "react";

import { Link } from "react-router-dom";

import { useTranslation } from "react-i18next";

import { easing } from "@mui/material/styles/createTransitions";

import Box from "@mui/material/Box";
import Button from "@mui/material/Button";
import Collapse from "@mui/material/Collapse";
import Grid from "@mui/material/Grid";
import Grow from "@mui/material/Grow";
import RenderCard from "../render-card-components/RenderCard";

import { Send } from "@mui/icons-material";

import { Project, Render } from "../../../../services/projects";
import { ColorMode, useColorMode } from "../../../../services/colorMode";

interface Props {
  project: Project;
  open: boolean;
}
export default function RenderCardGrid({ project, open }: Props) {
  const { t } = useTranslation();

  const [colorMode, _] = useColorMode();

  return (
    <Collapse in={open} unmountOnExit>
      <Grid
        container
        spacing={2}
        padding={2}
        bgcolor={colorMode === ColorMode.Dark ? "#2D2D2C" : "#EAEAEA"}
        width='95%'
        ml='5%'
        mt='0.5px'
        borderRadius={2}
        right={0}
      >
        <Box width='100%' paddingX={2} paddingTop={1}>
          <Button
            variant='outlined'
            endIcon={<Send />}
            color='inherit'
            component={Link}
            to={`/editor/${project.id}`}
            target='_blank'
            rel='noopener noreferrer'
          >
            {t("open_project_in_editor")}
          </Button>
        </Box>
        {project.renders.map((render: Render, index) => (
          <Grow
            key={render.id}
            in={open}
            style={{ transitionDelay: `${index * 40 + 40}ms` }}
            easing={easing.easeInOut}
          >
            <Grid item xs={6} sm={6} md={4} xl={3} key={render.id}>
              <RenderCard render={render} key={render.id} />
            </Grid>
          </Grow>
        ))}
      </Grid>
    </Collapse>
  );
}
