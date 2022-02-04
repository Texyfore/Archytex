import {
  Box,
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
  Button,
  Snackbar,
} from "@mui/material";
import React, { useEffect, useState } from "react";
import EditorMenu from "../components/editor-components/EditorMenu";
import EditorAppBar from "../components/editor-components/EditorAppBar";
import AppBarOffset from "../components/AppBarOffset";
import useDimensions from "react-cool-dimensions";
import Environment from "../env";
import {
  Grid3x3Rounded,
  Grid4x4Rounded,
  VideoCameraBack,
} from "@mui/icons-material";
import { useParams } from "react-router-dom";
import SelectTransformModeIcon from "../components/icons/SelectTransformModeIcon";
import MoveTransformModeIcon from "../components/icons/MoveTransformModeIcon";
import RotateTransformModeIcon from "../components/icons/RotateTransformModeIcon";
import ScaleTransformModeIcon from "../components/icons/ScaleTransformModeIcon";
import { useApi } from "../services/user/api";
import { useTranslation } from "react-i18next";
import EditorModeButtons from "../components/editor-components/EditorModeButtons";
import TranslateModeButtons from "../components/editor-components/TranslateModeButtons";
import CameraSettingsButton from "../components/editor-components/CameraSettingsButton";
import GridSettingsButton from "../components/editor-components/GridSettingsButton";
import MuiAlert, { AlertProps } from "@mui/material/Alert";

let browserEndpoint: any;
type EditorMode = "solid" | "face" | "vertex" | "prop";
type translateMode = "select" | "move" | "rotate" | "scale";
type libraryType = "textureLibrary" | "propLibrary";

let saveType: "export" | "save" | "render" = "save";

const Alert = React.forwardRef<HTMLDivElement, AlertProps>(function Alert(
  props,
  ref
) {
  return <MuiAlert elevation={6} ref={ref} variant='filled' {...props} />;
});

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

  //Camera speed
  const [cameraSpeed, setCameraSpeed] = useState(50);
  const handleCameraSpeedChange = (e: any) => {
    setCameraSpeed(e.target.value);
  };

  // Grid settings
  const [gridStep, setGridStep] = useState<number>(100);
  const handleGridStepChange = (e: any) => {
    setGridStep(e.target.value);
  };

  // Error snackbar
  const [snackBarError, setSnackBarError] = useState("");
  const handleSnackBarClose = (e: any) => {
    setSnackBarError("");
  };

  return (
    <>
      <EditorAppBar onSave={handleAppBarButtonClick} />
      <AppBarOffset variant='dense' />
      <Box display='flex' height={`calc(100vh - 48px)`} overflow='hidden'>
        <Box
          width='100%'
          height='100%'
          ref={observe}
          sx={{ backgroundColor: "#0c0c0c" }}
        />
        <EditorMenu libraryType={libraryType} />
      </Box>
      <canvas
        id='viewport-canvas'
        style={{ position: "absolute", top: `48px` }}
        onContextMenu={(e) => {
          e.preventDefault();
        }}
      ></canvas>

      {/* viewport UI */}
      <EditorModeButtons
        editorMode={editorMode}
        handleEditorModeChange={handleEditorModeChange}
      />

      <TranslateModeButtons />

      <CameraSettingsButton
        cameraSpeed={cameraSpeed}
        handleCameraSpeedChange={handleCameraSpeedChange}
      />

      <GridSettingsButton
        gridStep={gridStep}
        handleGridStepChange={handleGridStepChange}
      />

      <Snackbar
        open={snackBarError !== ""}
        autoHideDuration={6000}
        onClose={handleSnackBarClose}
      >
        <Alert
          onClose={handleSnackBarClose}
          severity='error'
          sx={{ width: "100%" }}
        >
          {snackBarError}
        </Alert>
      </Snackbar>
    </>
  );
}
