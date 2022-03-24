import React, { useState } from "react";

import { useTranslation } from "react-i18next";

import Box from "@mui/material/Box";
import Card from "@mui/material/Card";
import CardActionArea from "@mui/material/CardActionArea";
import CardMedia from "@mui/material/CardMedia";
import CardContent from "@mui/material/CardContent";
import Tooltip from "@mui/material/Tooltip";
import Typography from "@mui/material/Typography";
import Skeleton from "@mui/material/Skeleton";

import CircularProgressWithLabel from "../../general-components/CircularProgressWithLabel";
import EnlargeImageModal from "./EnlargeImageModal";
import RenderCardActions from "./RenderCardActions";

import { Render, Project } from "../../../services/projects";

import Environment from "../../../env";

interface RenderCardProps {
  render: Render;
  project: Project;
}

export default function RenderCard({ render, project }: RenderCardProps) {
  const { t } = useTranslation();

  //Enlarge render image modal
  const [openEnlargeRenderModal, setOpenEnlargeRenderModal] = useState<
    undefined | Render
  >(undefined);
  const handleOpenEnlargeRenderModal = (render: Render) => {
    setOpenEnlargeRenderModal(render);
  };
  const handleCloseEnlargeRenderModal = () =>
    setOpenEnlargeRenderModal(undefined);

  return (
    <>
      <Card sx={{ maxWidth: 345 }} key={render.id}>
        <CardActionArea
          disabled={render.status < 1}
          onClick={() => handleOpenEnlargeRenderModal(render)}
        >
          {render.status < 1 ? (
            <Skeleton
              variant='rectangular'
              animation='wave'
              sx={{ height: { xs: "150px", sm: "200px", md: "200px" } }}
            />
          ) : (
            <CardMedia
              component='img'
              sx={{
                height: { xs: "150px", sm: "200px", md: "200px" },
              }}
              image={
                render.status < 1
                  ? ""
                  : `${Environment.base_url}render/${render.icon}`
              }
              alt='archytex_render'
            />
          )}
          {/* Image overlay for progress information */}
          {render.status < 1 && (
            <Box position='relative' width='100%' height={0}>
              <Box
                position='absolute'
                top={{ xs: "-150px", sm: "-200px", md: "-200px" }}
                height={{ xs: "150px", sm: "200px", md: "200px" }}
                width='100%'
                display='flex'
                justifyContent='center'
                alignItems='center'
                bgcolor='rgba(0, 0, 0, 0.7)'
              >
                <Box>
                  {render.status === 0 ? (
                    <Typography color='#F4F4F4'>{t("in_queue")}</Typography>
                  ) : (
                    <CircularProgressWithLabel
                      size={80}
                      thickness={1}
                      value={Math.floor(render.status * 100)}
                    />
                  )}
                </Box>
              </Box>
            </Box>
          )}
          <Tooltip title={render.name} placement='top'>
            <CardContent sx={{ maxHeight: "100px" }}>
              <Typography variant='h6' component='div' noWrap>
                {render.name}
              </Typography>
            </CardContent>
          </Tooltip>
        </CardActionArea>
        <RenderCardActions render={render} project={project} />
      </Card>

      {/* Enlarge render image modal */}
      <EnlargeImageModal
        render={openEnlargeRenderModal}
        handleClose={handleCloseEnlargeRenderModal}
      />
    </>
  );
}
