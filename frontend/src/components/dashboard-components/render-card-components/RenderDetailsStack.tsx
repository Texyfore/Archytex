import React from "react";

import { useTranslation } from "react-i18next";

import { styled } from "@mui/material/styles";
import Paper from "@mui/material/Paper";
import Stack from "@mui/material/Stack";
import Typography from "@mui/material/Typography";
import Divider from "@mui/material/Divider";

import { Render } from "../../../services/projects";

const Item = styled(Paper)(({ theme }) => ({
  ...theme.typography.body1,
  padding: theme.spacing(1),
  textAlign: "center",
  color: theme.palette.text.secondary,
}));

interface Props {
  render: Render;
}
export default function RenderDetailsStack({ render }: Props) {
  const { t } = useTranslation();

  return (
    <div>
      <Stack spacing={2}>
        <Divider />
        <Typography variant='h5' textAlign='center'>
          {render.name}
        </Typography>
        <Divider />
        <Typography textAlign='center'>
          {t("render_status").toUpperCase()}
        </Typography>
        <Item>
          {render.status === 1 ? t("finished") : `${render.status * 100}%`}
        </Item>
        <Typography textAlign='center'>{t("started").toUpperCase()}</Typography>
        <Item>
          {(render.started + "").split(" ").map((string, index) => {
            if (index < 5) {
              return <span>{string} </span>;
            }
            return "";
          })}
        </Item>
        <Typography textAlign='center'>
          {t("finished").toUpperCase()}
        </Typography>
        <Item>
          {(render.finished + "").split(" ").map((string, index) => {
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
