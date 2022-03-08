import { useCallback, useEffect, useState } from "react";

import { useParams } from "react-router-dom";

import useDimensions from "react-cool-dimensions";

import Box from "@mui/material/Box";

import EditorAppBar from "../components/editor-components/EditorAppBar";
import EditorModeButtons from "../components/editor-components/EditorModeButtons";

import useNotification from "../services/hooks/useNotification";
import { useApi } from "../services/user/api";
import { useTranslation } from "react-i18next";
import { getAssets, Prop, Texture } from "../services/Library";

type EditorMode = "solid" | "face" | "vertex" | "prop";

let current_event = 0;
let listeners: { [key: number]: (value: Uint8Array) => void } = {};
let rightDown = false;

export default function Editor() {
  const { t } = useTranslation();
  // Get project ID
  const { projectId } = useParams<{ projectId: string }>();

  const api = useApi(false);

  // Selected texture
  const [_assets, _] = useState(() => getAssets());
  const [textures, setTextures] = useState<Texture[]>([]);
  const [props, setProps] = useState<Prop[]>([]);
  useEffect(() => {
    (async () => {
      const assets = await _assets;
      setTextures(assets.textures);
      setProps(assets.props);
    })()
  }, []);
  const [texture, setTexture] = useState<Texture | null>(textures[0]);
  useEffect(() => {
    setTexture(textures[0]);
  }, textures)
  const handleTextureChange = (texture: Texture) => {
    setTexture(texture);
  };
  const [prop, setProp] = useState<Prop | null>(null);
  useEffect(() => {
    setProp(props[0]);
  }, props)
  const handlePropChange = (prop: Prop) => {
    setProp(prop);
  };

  const [sender, setSender] = useState<any | null>(null);
  const [width, setWidth] = useState(1);
  const [height, setHeight] = useState(1);

  const { observe } = useDimensions({
    onResize: ({ width, height }) => {
      setWidth(Math.ceil(width));
      setHeight(Math.floor(height));
    },
  });

  useEffect(() => {
    if (sender !== null) {
      sender.setResolution(
        width * window.devicePixelRatio,
        height * window.devicePixelRatio
      );
    }
  }, [width, height, sender]);


  useEffect(() => {
    import("viewport").then((viewport) => {
      const channel = new viewport.Channel();
      setSender(channel.sender());
      const callback = new viewport.Callback(
        (id: number, scene: Uint8Array) => {
          listeners[id](scene)
        },
        (modeIndex: number) => {
          let mode: EditorMode = "solid";
          switch (modeIndex) {
            case 0:
              mode = "solid";
              break;
            case 1:
              mode = "face";
              break;
            case 2:
              mode = "vertex";
              break;
            case 3:
              mode = "prop";
              break;
            default:
              mode = "solid";
              break;
          }
          handleEditorModeChange(mode);
          console.log(`mode ${mode}`);
        }
      );


      viewport.run(channel, callback);
    });
  }, []);

  useEffect(() => {
    if (sender === null) {
      return;
    }
    const canvas = document.getElementById("viewport-canvas");

    if (canvas !== null) {
      canvas.addEventListener("mousedown", ev => {
        if (ev.button === 2) {
          canvas.requestPointerLock();
          sender.setPointerLock(true);
          rightDown = true;
        }
      });

      canvas.addEventListener("mouseup", ev => {
        if (ev.button == 2) {
          document.exitPointerLock();
          sender.setPointerLock(false);
          rightDown = false;
        }
      })

      canvas.addEventListener("mousemove", ev => {
        if (rightDown) {
          sender.movement(ev.movementX, ev.movementY);
        }
      });
    }
  }, [sender])

  useEffect(() => {
    (async () => {
      if (api?.state === "logged-in" && sender !== null) {
        const data = await api.load(projectId);
        if (data !== undefined && data.length > 0) {
          sender.loadScene(data);
        }
      }
    })()
  }, [api, sender]);

  let save = useCallback(() => new Promise((resolve: (value: Uint8Array) => void) => {
    const n = current_event;
    current_event++;
    listeners[n] = resolve;
    console.log(`Sending save request #${n}`)
    sender.saveScene(n);
  }), [sender]);

  const onRender = async (width: number, height: number, samples: number) => {
    if (api?.state == "logged-in") {
      addNotification(t("rendering_started"), "info");
      const data = await save();
      await api.render(data, projectId, width, height, samples);
    } else {
      addNotification(t("not_logged_in"), "error")
    }
  }

  // App bar button click
  const handleAppBarButtonClick = async (type: "export" | "save") => {
    console.log("Got Save event")
    const data = await save();
    switch (type) {
      case "export":

        break;
      case "save":
        if (api?.state == "logged-in") {
          await (api.save(data, projectId).catch(() => {
            addNotification("Could not save project", "error")
          }));
        } else {
          addNotification("You are not logged in.", "error")
        }
        break;
    }
  };

  //Editor mode
  const [editorMode, setEditorMode] = useState<EditorMode>("solid");
  const handleEditorModeChange = (mode: EditorMode, send: boolean = false) => {
    if (mode != null) {
      //solid : 0
      //face : 1
      //vertex : 2
      //prop : 3
      //move : 4
      //rotate : 5
      setEditorMode(mode);
      if (send) {
        let modeIndex = 0;
        switch (mode) {
          case "solid":
            modeIndex = 0;
            break;
          case "face":
            modeIndex = 1;
            break;
          case "vertex":
            modeIndex = 2;
            break;
          case "prop":
            modeIndex = 3;
            break;
          default:
            break;
        }
        sender.button(modeIndex);
      }
    }
  };

  // Error display
  const { addNotification } = useNotification();

  return (
    <>
      <EditorAppBar onSave={handleAppBarButtonClick} onRender={onRender} />
      <Box width='100%' height='48px'></Box>

      <Box display='flex' height={`calc(100vh - 48px)`} overflow='hidden'>
        <Box width='100%' height='100%' ref={observe} bgcolor='#0c0c0c' />
      </Box>

      <canvas
        id='viewport-canvas'
        style={{
          position: "absolute",
          top: "48px",
        }}
        onContextMenu={(e) => {
          e.preventDefault();
        }}
      ></canvas>

      {/* viewport UI */}
      <EditorModeButtons
        editorMode={editorMode}
        handleEditorModeChange={handleEditorModeChange}
      />
    </>
  );
}
