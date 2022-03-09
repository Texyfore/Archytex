import { useCallback, useEffect, useState } from "react";

import { useParams } from "react-router-dom";

import { useTranslation } from "react-i18next";

import useDimensions from "react-cool-dimensions";

import Box from "@mui/material/Box";

import EditorAppBar from "../components/editor-components/EditorAppBar";
import EditorModeButtons from "../components/editor-components/EditorModeButtons";

import useNotification from "../services/hooks/useNotification";
import { useApi } from "../services/user/api";
import { getAssets, Prop, Texture } from "../services/Library";
import EditorMenu from "../components/editor-components/EditorMenu";
import Environment from "../env";

type EditorMode = "solid" | "face" | "vertex" | "prop";

let vp: any = null;

let current_event = 0;
let listeners: { [key: number]: (value: Uint8Array) => void } = {};
let rightDown = false;
let loadedTextures = new Set<number>();
let loadedProps = new Set<number>();

export default function Editor() {
  const { t } = useTranslation();
  // Get project ID
  const { projectId } = useParams<{ projectId: string }>();

  const api = useApi(false);

  // Initialise sender
  const [sender, setSender] = useState<any | null>(null);

  // Asset management
  const [_assets, _] = useState(() => getAssets());

  const [textures, setTextures] = useState<Texture[]>([]);
  const [props, setProps] = useState<Prop[]>([]);

  // Fetch assets, set textures and props
  useEffect(() => {
    (async () => {
      const assets = await _assets;
      setTextures(assets.textures);
      setProps(assets.props);
    })();
  }, []);

  // Selected texture
  const [texture, setTexture] = useState<Texture>(textures[0]);
  useEffect(() => {
    setTexture(textures[0]);
  }, [textures]);
  const handleTextureChange = (texture: Texture) => {
    setTexture(texture);
  };

  useEffect(() => {
    if (sender !== null && texture !== undefined) {
      fetchBytes(`${Environment.asset_url}/textures/${texture.name}.png`).then((buffer) => {
        if (!loadedTextures.has(texture.id)) {
          sender.loadTexture(texture.id, buffer);
          sender.setTexture(texture.id);
        }
      });
    }
  }, [texture, sender]);

  // Selected prop
  const [prop, setProp] = useState<Prop>(props[0]);
  useEffect(() => {
    setProp(props[0]);
  }, [props]);
  const handlePropChange = (prop: Prop) => {
    setProp(prop);
  };

  useEffect(() => {
    if (sender !== null && prop !== undefined) {
      sender.setProp(prop.id);
      fetchBytes(`${Environment.asset_url}/props/${prop.name}.amdl`).then((buf) => {
        if (!loadedProps.has(prop.id)) {
          sender.loadProp(prop.id, buf);
          loadedProps.add(prop.id);
        }
      });

      prop.dependencies.forEach(dep => {
        fetchBytes(`${Environment.asset_url}/textures/${dep}.png`)
          .then(buf => {
            const id = textures.find(tex => { return tex.name === dep; })?.id;
            if (id !== undefined) {
              if (!loadedTextures.has(id)) {
                sender.loadTexture(id, buf);
              }
            }
          })
      });
    }
  }, [prop, sender, textures]);

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
      vp = viewport;

      const channel = new vp.Channel();
      setSender(channel.sender());

      const callback = new vp.Callback(
        (id: number, scene: Uint8Array) => {
          listeners[id](scene);
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
        }
      );

      vp.run(channel, callback);
    });
  }, []);

  useEffect(() => {
    if (sender === null) {
      return;
    }

    const canvas = document.getElementById("viewport-canvas");

    if (canvas !== null) {
      canvas.addEventListener("mousedown", (ev) => {
        if (ev.button === 2) {
          canvas.requestPointerLock();
          sender.setPointerLock(true);
          rightDown = true;
        }
      });

      canvas.addEventListener("mouseup", (ev) => {
        if (ev.button === 2) {
          document.exitPointerLock();
          sender.setPointerLock(false);
          rightDown = false;
        }
      });

      canvas.addEventListener("mousemove", (ev) => {
        if (rightDown) {
          sender.movement(ev.movementX, ev.movementY);
        }
      });
    }
  }, [sender]);

  useEffect(() => {
    (async () => {
      if (api?.state === "logged-in" && sender !== null) {
        const data = await api.load(projectId);
        if (data !== undefined && data.length > 0) {
          const scene = new vp.Scene(data);
          if (scene !== undefined) {
            scene.textures().forEach((id: number) => {
              (async () => {
                const texture = textures.find(texture => texture.id == id);
                if (texture !== undefined) {
                  const bytes = await fetchBytes(
                    `${Environment.asset_url}/textures/${texture.name}.png`
                  );
                  scene.loadTexture(texture.id, bytes);
                }
              })();
            })

            scene.props().foreach((id: number) => {
              (
                async () => {
                  const prop = props.find((prop) => { return prop.id == id });
                  if (prop !== undefined) {
                    await Promise.all(
                      prop.dependencies.map((dep) => {
                        return async () => {
                          const texture = textures.find(texture => texture.name == dep);
                          if (texture !== undefined) {
                            const bytes = await fetchBytes(
                              `${Environment.asset_url}/textures/${texture.name}.png`
                            );
                            scene.loadTexture(texture.id, bytes);
                          }
                        };
                      })
                    );

                    const bytes = await fetchBytes(
                      `${Environment.asset_url}/props/${prop.name}.amdl`
                    );
                    scene.loadProp(prop.id, bytes);
                  }
                }
              )();
            });

            sender.loadScene(scene);
          }
        }
      }
    })();
  }, [api, sender, textures, props]);

  let save = useCallback(
    () =>
      new Promise((resolve: (value: Uint8Array) => void) => {
        const n = current_event;
        current_event++;
        listeners[n] = resolve;
        console.log(`Sending save request #${n}`);
        sender.saveScene(n);
      }),
    [sender]
  );

  const onRender = async (width: number, height: number, samples: number) => {
    if (api?.state == "logged-in") {
      addNotification(t("rendering_started"), "info");
      const data = await save();
      await api.render(data, projectId, width, height, samples);
    } else {
      addNotification(t("not_logged_in"), "error");
    }
  };

  // App bar button click
  const handleAppBarButtonClick = async (type: "export" | "save") => {
    console.log("Got Save event");
    const data = await save();
    switch (type) {
      case "export":
        break;
      case "save":
        if (api?.state == "logged-in") {
          await api.save(data, projectId).catch(() => {
            addNotification("Could not save project", "error");
          });
        } else {
          addNotification("You are not logged in.", "error");
        }
        break;
    }
  };

  //Editor mode
  const [editorMode, setEditorMode] = useState<EditorMode>("solid");
  const handleEditorModeChange = (mode: EditorMode, send: boolean = false) => {
    if (mode != null) {
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
        {texture !== undefined && prop !== undefined && (
          <EditorMenu
            texture={texture}
            handleTextureChange={handleTextureChange}
            prop={prop}
            handlePropChange={handlePropChange}
            textures={textures}
            props={props}
          />
        )}
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

async function fetchBytes(url: string): Promise<Uint8Array> {
  const res = await fetch(url);
  const arrayBuffer = await res.arrayBuffer();
  return new Uint8Array(arrayBuffer);
}