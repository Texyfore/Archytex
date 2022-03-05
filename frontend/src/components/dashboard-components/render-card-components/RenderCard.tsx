import React, { useState } from "react";

import Box from "@mui/material/Box";
import Card from "@mui/material/Card";
import CardActionArea from "@mui/material/CardActionArea";
import CardMedia from "@mui/material/CardMedia";
import CardContent from "@mui/material/CardContent";
import Tooltip from "@mui/material/Tooltip";
import Typography from "@mui/material/Typography";

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
          <CardMedia
            component='img'
            sx={{
              height: { xs: "150px", sm: "200px", md: "250px" },
            }}
            image={`${Environment.base_url}render/${render.icon}`}
            alt='archytex_render'
          />
          {/* Image overlay for progress information */}
          <Box
            position='relative'
            width='100%'
            height={0}
            display={render.status < 100 ? "block" : "none"}
          >
            <Box
              position='absolute'
              top={{ xs: "-150px", sm: "-200px", md: "-250px" }}
              height={{ xs: "150px", sm: "200px", md: "250px" }}
              width='100%'
              display='flex'
              justifyContent='center'
              alignItems='center'
              bgcolor='rgba(0, 0, 0, 0.7)'
            >
              <Box>
                <CircularProgressWithLabel
                  size={80}
                  thickness={1}
                  value={render.status * 100}
                />
              </Box>
            </Box>
          </Box>
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
