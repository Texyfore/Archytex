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
import EditorHandle from "../editorUtils";
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

const appBarHeight = 48;
let editorHandle: EditorHandle;
type viewportMode = "solid" | "prop";
type selectionMode = "mesh" | "face" | "vertex";
type translateMode = "select" | "move" | "rotate" | "scale";
type libraryType = "textureLibrary" | "propLibrary";

let saveType: "export" | "save" | "render" = "save";

export default function Editor() {
  // Use API
  const api = useApi();

  // Get project ID
  const { projectId } = useParams<{ projectId: string }>();

  // Library type
  const [libraryType, setLibraryType] = useState<libraryType>("textureLibrary");

  // App bar button click
  const handleAppBarButtonClick = (type: "export" | "save" | "render") => {
    saveType = type;
    editorHandle.saveScene(type);
  };
  const { observe } = useDimensions({
    onResize: ({ width, height }) => {
      editorHandle.setResolution(width, height);
    },
  });
  useEffect(() => {
    editorHandle = new EditorHandle({
      editorModeChanged: (mode) => {
        let stringMode: viewportMode = "solid";

        switch (mode) {
          case 0:
            stringMode = "solid";
            setLibraryType("textureLibrary");
            break;
          case 1:
            stringMode = "prop";
            setLibraryType("propLibrary");
            break;
        }
        setViewportMode(stringMode);
      },
      solidEditorModeChanged: (mode) => {
        let stringMode: selectionMode = "mesh";
        switch (mode) {
          case 0:
            stringMode = "mesh";
            break;
          case 1:
            stringMode = "face";
            break;
          case 2:
            stringMode = "vertex";
            break;
        }
        setSelectionMode(stringMode);
      },
      gizmoChanged: (gizmo) => {
        let stringMode: translateMode = "select";

        switch (gizmo) {
          case 0:
            stringMode = "select";
            break;
          case 1:
            stringMode = "move";
            break;
          case 2:
            stringMode = "rotate";
            break;
          case 3:
            stringMode = "scale";
            break;
        }
        setTranslateMode(stringMode);
      },
      cameraSpeedChanged: (speed) => {
        const newSpeed = Math.round(10.4921 * Math.log(14.6738 * speed));
        setCameraSpeed(newSpeed);
      },
      gridSizeChanged: (size) => {
        setGridRes(3 - size);
      },
      sceneSaved: (scene) => {
        switch (saveType) {
          case "export":
            const blob = new Blob([scene]);
            const blobUrl = URL.createObjectURL(blob);
            const link = document.createElement("a");
            link.href = blobUrl;
            link.download = "scene.ascn";
            document.body.appendChild(link);
            link.dispatchEvent(
              new MouseEvent("click", {
                bubbles: true,
                cancelable: true,
                view: window,
              })
            );
            document.body.removeChild(link);
            break;
          case "save":
            if (api?.state === "logged-in") {
              api.save(scene, projectId);
            }
            break;
          case "render":
            if (api?.state === "logged-in") {
              api.render(scene, projectId);
            }
            break;
          default:
            break;
        }
      },
    });

    (async () => {
      const res = await fetch(`${Environment.asset_url}/dummydb.json`);
      const assets = await res.json();
      editorHandle.loadTextures(assets.textures);
      editorHandle.loadProps(assets.props);
    })();

    return editorHandle.destroy;
  }, [api]);

  // Viewport mode change
  const [viewportMode, setViewportMode] = useState<viewportMode>("solid");
  const handleViewportModeChange = (e: any) => {
    setViewportMode(e.target.value);
    editorHandle.setEditorMode(e.target.value === "solid" ? 0 : 1);
  };

  // Selection mode change
  const [selectionMode, setSelectionMode] =
    React.useState<selectionMode>("mesh");

  const handleSelectionModeChange = (
    event: React.MouseEvent<HTMLElement> | undefined,
    newSelectionMode: selectionMode | null
  ) => {
    if (newSelectionMode != null) {
      setSelectionMode(newSelectionMode);
      let id = -1;
      switch (newSelectionMode) {
        case "mesh":
          id = 0;
          break;
        case "face":
          id = 1;
          break;
        case "vertex":
          id = 2;
          break;

        default:
          break;
      }

      editorHandle.setSolidEditorMode(id);
    }
  };

  // Translate mode change
  const [translateMode, setTranslateMode] =
    React.useState<translateMode>("select");
  const handleTranslateModeChange = (
    event: React.MouseEvent<HTMLElement>,
    newTranslateMode: translateMode | null
  ) => {
    if (newTranslateMode != null) {
      setTranslateMode(newTranslateMode);
      let id = -1;
      switch (newTranslateMode) {
        case "select":
          id = 0;
          break;
        case "move":
          id = 1;
          break;
        case "rotate":
          id = 2;
          break;
        case "scale":
          id = 3;
          break;
        default:
          break;
      }
      editorHandle.setGizmo(id);
    }
  };

  // Camera settings
  const [cameraAnchorEl, setCameraAnchorEl] =
    React.useState<null | HTMLElement>(null);
  const cameraMenuOpen = Boolean(cameraAnchorEl);
  const handleCameraMenuClick = (event: React.MouseEvent<HTMLElement>) => {
    setCameraAnchorEl(event.currentTarget);
  };
  const handleCameraMenuClose = () => {
    setCameraAnchorEl(null);
  };
  const [cameraSpeed, setCameraSpeed] = useState<number>(50);
  const handleCameraSpeedChange = (
    event: Event,
    value: number | number[],
    activeThumb: number
  ) => {
    if (typeof value === "number") {
      setCameraSpeed(value);
      editorHandle.setCameraSpeed(value);
    } else {
      editorHandle.setCameraSpeed(value[0]);
    }
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
  const [gridRes, setGridRes] = useState<number>(3);
  const handleGridResChange = (
    event: Event,
    value: number | number[],
    activeThumb: number
  ) => {
    if (typeof value === "number") {
      setGridRes(value);
      editorHandle.setGridSize((1 - (value - 1) * 0.2) * 5 - 3);
    } else {
      editorHandle.setGridSize((1 - (value[0] - 1) * 0.2) * 5 - 3);
    }
  };

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
        {/* Viewport mode */}
        <Box position='absolute' top={appBarHeight + 10} left={10} width={180}>
          <Select
            id='mode-select'
            value={viewportMode}
            onChange={handleViewportModeChange}
            size='small'
            fullWidth
            sx={{
              color: "white",
              height: 30.75,
              underline: {
                "&:after": {
                  borderBottom: "1px solid pink",
                  borderTop: "1px solid pink",
                },
              },
            }}
          >
            <MenuItem value='prop'>
              <Box display='flex' alignItems='center' gap={2}>
                <Chair fontSize='small' /> Prop mode
              </Box>
            </MenuItem>
            <MenuItem value='solid'>
              <Box display='flex' alignItems='center' gap={2}>
                <ViewCompact fontSize='small' /> Solid mode
              </Box>
            </MenuItem>
          </Select>
        </Box>

        {/* Selection mode */}
        <Box
          position='absolute'
          top={appBarHeight + 10}
          left={220}
          display={viewportMode === "solid" ? "initial" : "none"}
        >
          <ToggleButtonGroup
            value={selectionMode}
            exclusive
            onChange={handleSelectionModeChange}
            color='primary'
            size='small'
            sx={{ height: 30.75 }}
          >
            <ToggleButton value='mesh'>
              <Tooltip title='Mesh select mode'>
                <Box marginTop={0.8}>
                  <MeshSelectIcon />
                </Box>
              </Tooltip>
            </ToggleButton>
            <ToggleButton value='face'>
              <Tooltip title='Face select mode'>
                <Box marginTop={0.8}>
                  <FaceSelectIcon />
                </Box>
              </Tooltip>
            </ToggleButton>
            <ToggleButton value='vertex'>
              <Tooltip title='Vertex select mode'>
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
            value={translateMode}
            exclusive
            onChange={handleTranslateModeChange}
            color='primary'
            size='small'
            orientation='vertical'
          >
            <ToggleButton value='select'>
              <Tooltip title='Select' placement='right'>
                <Box marginTop={0.8} width={36} height={30}>
                  <SelectTransformModeIcon />
                </Box>
              </Tooltip>
            </ToggleButton>
            <ToggleButton value='move'>
              <Tooltip title='Move' placement='right'>
                <Box marginTop={0.8} width={36} height={30}>
                  <MoveTransformModeIcon />
                </Box>
              </Tooltip>
            </ToggleButton>
            <ToggleButton value='rotate'>
              <Tooltip title='Rotate' placement='right'>
                <Box marginTop={0.8} width={36} height={30}>
                  <RotateTransformModeIcon />
                </Box>
              </Tooltip>
            </ToggleButton>
            <ToggleButton value='scale'>
              <Tooltip title='Scale' placement='right'>
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
          <Tooltip title='Camera settings'>
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
                <Typography gutterBottom>Camera speed</Typography>
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
          <Tooltip title='Grid settings'>
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
                <Typography gutterBottom>Grid resolution</Typography>
                <Box width={150}>
                  <Stack
                    spacing={2}
                    direction='row'
                    sx={{ mb: 1 }}
                    alignItems='center'
                  >
                    <Grid3x3Rounded fontSize='small' />
                    <Slider
                      size='small'
                      defaultValue={gridRes}
                      step={1}
                      min={1}
                      max={6}
                      onChange={handleGridResChange}
                      valueLabelDisplay='auto'
                    />
                    <Grid4x4Rounded fontSize='small' />
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
