import {
  Box,
  Select,
  MenuItem,
  Tooltip,
  IconButton,
  Menu,
  Slider,
  List,
  ListItem,
  ToggleButtonGroup,
  ToggleButton,
  Stack,
  Typography,
} from "@mui/material";
import React, { useEffect, useState } from "react";
import EditorMenu from "../components/editor-components/EditorMenu";
import EditorAppBar from "../components/editor-components/EditorAppBar";
import AppBarOffset from "../components/AppBarOffset";
import useDimensions from "react-cool-dimensions";
import Environment from "../env";
import {
  Chair,
  Grid3x3Rounded,
  Grid4x4Rounded,
  VideoCameraBack,
  ViewCompact,
} from "@mui/icons-material";
import MeshSelectIcon from "../components/icons/MeshSelectIcon";
import FaceSelectIcon from "../components/icons/FaceSelectIcon";
import VertexSelectIcon from "../components/icons/VertexSelectIcon";
import { useParams } from "react-router-dom";
import SelectTransformModeIcon from "../components/icons/SelectTransformModeIcon";
import MoveTransformModeIcon from "../components/icons/MoveTransformModeIcon";
import RotateTransformModeIcon from "../components/icons/RotateTransformModeIcon";
import ScaleTransformModeIcon from "../components/icons/ScaleTransformModeIcon";
import { useApi } from "../services/user/api";
import { useTranslation } from "react-i18next";

const appBarHeight = 48;
let browserEndpoint: any;
type EditorMode = "solid" | "face" | "vertex" | "prop";
type translateMode = "select" | "move" | "rotate" | "scale";
type libraryType = "textureLibrary" | "propLibrary";

let saveType: "export" | "save" | "render" = "save";

export default function Editor() {
  // Use i18n
  const { t } = useTranslation();
  const meshTooltipText = t("mesh_select_mode");
  const propTooltipText = t("prop_mode");
  const faceTooltipText = t("face_select_mode");
  const vertexTooltipText = t("vertex_select_mode");
  const selectTooltipText = t("select_translate_mode");
  const moveTooltipText = t("move_translate_mode");
  const rotateTooltipText = t("rotate_translate_mode");
  const scaleTooltipText = t("scale_translate_mode");
  const cameraSettingsTooltipText = t("camera_settings");
  const gridSettingsTooltipText = t("grid_settings");

  // Use API
  const api = useApi();

  // Get project ID
  const { projectId } = useParams<{ projectId: string }>();

  // Library type
  const [libraryType, setLibraryType] = useState<libraryType>("textureLibrary");

  // App bar button click
  const handleAppBarButtonClick = (type: "export" | "save" | "render") => {
    saveType = type;
    // editorHandle.saveScene(type);
  };
  const { observe } = useDimensions({
    onResize: ({ width, height }) => {
      browserEndpoint.setResolution(width, height);
    },
  });
  useEffect(() => {
    import("viewport").then((viewport) => {
      const channel = new viewport.Channel();
      const wasmEndPoint = channel.wasmEndpoint(
        (editorModeId: number) => {
          switch (editorModeId) {
            case 0:
              setEditorMode("solid");
              break;
            case 1:
              setEditorMode("face");
              break;
            case 2:
              setEditorMode("vertex");
              break;
            case 3:
              setEditorMode("prop");
              break;

            default:
              break;
          }
        },
        (speed: number) => {
          setCameraSpeed(speed);
        },
        (step: number) => {}
      );
      browserEndpoint = channel.browserEndpoint();
      viewport.run(wasmEndPoint);
    });
  }, []);

  //Editor mode
  const [editorMode, setEditorMode] = useState<EditorMode>("solid");
  const handleEditorModeChange = (e: any) => {
    if (e.target.value != null) {
      setEditorMode(e.target.value);
    }
  };

  //Camera settings
  const [cameraAnchorEl, setCameraAnchorEl] =
    React.useState<null | HTMLElement>(null);
  const cameraMenuOpen = Boolean(cameraAnchorEl);
  const handleCameraMenuClick = (event: React.MouseEvent<HTMLElement>) => {
    setCameraAnchorEl(event.currentTarget);
  };
  const handleCameraMenuClose = () => {
    setCameraAnchorEl(null);
  };

  //Camera speed
  const [cameraSpeed, setCameraSpeed] = useState(50);
  const handleCameraSpeedChange = (e: any) => {
    setCameraSpeed(e.target.value);
  };

  // Grid settings
  const [gridAnchorEl, setGridAnchorEl] = React.useState<null | HTMLElement>(
    null
  );
  const gridMenuOpen = Boolean(gridAnchorEl);
  const handleGridMenuClick = (event: React.MouseEvent<HTMLElement>) => {
    setGridAnchorEl(event.currentTarget);
  };
  const handleGridMenuClose = () => {
    setGridAnchorEl(null);
  };

  const [gridStep, setGridStep] = useState<number>(100);
  const handleGridStepChange = (e: any) => {};

  return (
    <React.Fragment>
      <EditorAppBar onSave={handleAppBarButtonClick} />
      <AppBarOffset variant='dense' />
      <Box
        display='flex'
        height={`calc(100vh - ${appBarHeight}px)`}
        overflow='hidden'
      >
        <Box
          width='100%'
          height='100%'
          ref={observe}
          sx={{ backgroundColor: "#0c0c0c" }}
        ></Box>
        <EditorMenu libraryType={libraryType} />
      </Box>
      <canvas
        id='viewport-canvas'
        style={{ position: "absolute", top: `${appBarHeight}px` }}
        onContextMenu={(e) => {
          e.preventDefault();
        }}
      ></canvas>

      {/* viewport UI */}
      <>
        {/* Editor mode */}
        <Box position='absolute' top={appBarHeight + 10} left={220}>
          <ToggleButtonGroup
            value={editorMode}
            exclusive
            onChange={handleEditorModeChange}
            color='primary'
            size='small'
            sx={{ height: 30.75 }}
          >
            <ToggleButton value='prop'>
              <Tooltip title={propTooltipText}>
                <Box marginTop={0.8}>
                  <Chair />
                </Box>
              </Tooltip>
            </ToggleButton>
            <ToggleButton value='solid'>
              <Tooltip title={meshTooltipText}>
                <Box marginTop={0.8}>
                  <MeshSelectIcon />
                </Box>
              </Tooltip>
            </ToggleButton>
            <ToggleButton value='face'>
              <Tooltip title={faceTooltipText}>
                <Box marginTop={0.8}>
                  <FaceSelectIcon />
                </Box>
              </Tooltip>
            </ToggleButton>
            <ToggleButton value='vertex'>
              <Tooltip title={vertexTooltipText}>
                <Box marginTop={0.8}>
                  <VertexSelectIcon />
                </Box>
              </Tooltip>
            </ToggleButton>
          </ToggleButtonGroup>
        </Box>

        {/* Translate mode */}
        <Box position='absolute' top='40vh' left={10}>
          <ToggleButtonGroup
            value={() => {}}
            exclusive
            onChange={() => {}}
            color='primary'
            size='small'
            orientation='vertical'
          >
            <ToggleButton value='select'>
              <Tooltip title={selectTooltipText} placement='right'>
                <Box marginTop={0.8} width={36} height={30}>
                  <SelectTransformModeIcon />
                </Box>
              </Tooltip>
            </ToggleButton>
            <ToggleButton value='move'>
              <Tooltip title={moveTooltipText} placement='right'>
                <Box marginTop={0.8} width={36} height={30}>
                  <MoveTransformModeIcon />
                </Box>
              </Tooltip>
            </ToggleButton>
            <ToggleButton value='rotate'>
              <Tooltip title={rotateTooltipText} placement='right'>
                <Box marginTop={0.8} width={36} height={30}>
                  <RotateTransformModeIcon />
                </Box>
              </Tooltip>
            </ToggleButton>
            <ToggleButton value='scale'>
              <Tooltip title={scaleTooltipText} placement='right'>
                <Box marginTop={0.8} width={36} height={30}>
                  <ScaleTransformModeIcon />
                </Box>
              </Tooltip>
            </ToggleButton>
          </ToggleButtonGroup>
        </Box>

        {/* Camera settings */}
        <Box
          position='absolute'
          top={appBarHeight + 10}
          left='calc(100% - 400px)'
        >
          <Tooltip title={cameraSettingsTooltipText}>
            <IconButton onClick={handleCameraMenuClick}>
              <VideoCameraBack />
            </IconButton>
          </Tooltip>
        </Box>
        <Menu
          anchorEl={cameraAnchorEl}
          id='camera-menu'
          open={cameraMenuOpen}
          onClose={handleCameraMenuClose}
          PaperProps={{
            elevation: 0,
            sx: {
              overflow: "visible",
              filter: "drop-shadow(0px 2px 8px rgba(0,0,0,0.5))",
              mt: 1.5,
              borderRadius: 2,
              "&:before": {
                content: '""',
                display: "block",
                position: "absolute",
                top: 0,
                right: 25,
                width: 10,
                height: 10,
                bgcolor: "paper.background",
                transform: "translateY(-50%) rotate(45deg)",
                zIndex: 0,
              },
            },
          }}
          transformOrigin={{ horizontal: "right", vertical: "top" }}
          anchorOrigin={{ horizontal: "right", vertical: "bottom" }}
        >
          <List dense>
            <ListItem>
              <Box>
                <Typography gutterBottom>{t("camera_speed")}</Typography>
                <Box width={150}>
                  <Slider
                    size='small'
                    defaultValue={cameraSpeed}
                    step={1}
                    min={1}
                    max={100}
                    onChange={handleCameraSpeedChange}
                    valueLabelDisplay='auto'
                  />
                </Box>
              </Box>
            </ListItem>
          </List>
        </Menu>

        {/* Grid settings */}
        <Box
          position='absolute'
          top={appBarHeight + 10}
          left='calc(100% - 450px)'
        >
          <Tooltip title={gridSettingsTooltipText}>
            <IconButton onClick={handleGridMenuClick}>
              <Grid3x3Rounded />
            </IconButton>
          </Tooltip>
        </Box>
        <Menu
          anchorEl={gridAnchorEl}
          id='grid-menu'
          open={gridMenuOpen}
          onClose={handleGridMenuClose}
          PaperProps={{
            elevation: 0,
            sx: {
              overflow: "visible",
              filter: "drop-shadow(0px 2px 8px rgba(0,0,0,0.5))",
              mt: 1.5,
              borderRadius: 2,
              "&:before": {
                content: '""',
                display: "block",
                position: "absolute",
                top: 0,
                right: 25,
                width: 10,
                height: 10,
                bgcolor: "paper.background",
                transform: "translateY(-50%) rotate(45deg)",
                zIndex: 0,
              },
            },
          }}
          transformOrigin={{ horizontal: "right", vertical: "top" }}
          anchorOrigin={{ horizontal: "right", vertical: "bottom" }}
        >
          <List dense>
            <ListItem>
              <Box>
                <Typography gutterBottom>{t("grid_resolution")}</Typography>
                <Box width={150}>
                  <Stack
                    spacing={2}
                    direction='row'
                    sx={{ mb: 1 }}
                    alignItems='center'
                  >
                    <Grid4x4Rounded fontSize='small' />
                    <Slider
                      size='small'
                      defaultValue={gridStep}
                      step={gridStep === 1 ? 9 : 10}
                      min={1}
                      max={10000}
                      onChange={handleGridStepChange}
                      valueLabelDisplay='auto'
                    />
                    <Grid3x3Rounded fontSize='small' />
                  </Stack>
                </Box>
              </Box>
            </ListItem>
          </List>
        </Menu>
      </>
    </React.Fragment>
  );
}
