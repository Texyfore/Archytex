import React from "react";

import { useTranslation } from "react-i18next";

import { styled } from "@mui/material/styles";
import Paper from "@mui/material/Paper";
import Stack from "@mui/material/Stack";
import Typography from "@mui/material/Typography";
import Divider from "@mui/material/Divider";

import { Project } from "../../../services/projects";

const Item = styled(Paper)(({ theme }) => ({
  ...theme.typography.body1,
  padding: theme.spacing(1),
  textAlign: "center",
  color: theme.palette.text.secondary,
}));

interface Props {
  project: Project;
}
export default function ProjectDetailsStack({ project }: Props) {
  const { t } = useTranslation();

  return (
    <div>
      <Stack spacing={2}>
        <Divider />
        <Typography variant='h5' textAlign='center'>
          {project.title}
        </Typography>
        <Divider />
        <Typography textAlign='center'>{t("renders").toUpperCase()}</Typography>
        <Item>{project.renders.length}</Item>
        <Typography textAlign='center'>{t("created").toUpperCase()}</Typography>
        <Item>
          {(project.created + "").split(" ").map((string, index) => {
            if (index < 5) {
              return <span>{string} </span>;
            }
            return "";
          })}
        </Item>
      </Stack>
    </div>
  );
}
