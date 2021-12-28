interface Callbacks {
  editorModeChanged: (mode: number) => void;
  solidEditorModeChanged: (mode: number) => void;
}

export type { Callbacks };

export default class EditorHandle {
  private callbacks: Callbacks;
  private loopTimeout: NodeJS.Timeout | undefined;
  private actionQueue: any[];
  private savedScene: Uint8Array | undefined;

  constructor(callbacks: Callbacks) {
    this.callbacks = callbacks;
    this.loopTimeout = undefined;
    this.actionQueue = [];
    this.savedScene = undefined;

    let initialized = false;

    import("viewport").then((module) => {
      this.loopTimeout = setInterval(() => {
        while (true) {
          const message = module.queryMessage();
          if (message !== undefined) {
            const json = JSON.parse(message);
            if (json !== undefined) {
              switch (json.message) {
                case "init": {
                  initialized = true;
                  break;
                }
                case "set-editor-mode": {
                  this.callbacks.editorModeChanged(json.mode);
                  break;
                }
                case "set-solid-editor-mode": {
                  this.callbacks.solidEditorModeChanged(json.mode);
                  break;
                }
              }
            }
          } else {
            break;
          }
        }

        if (!initialized) {
          return;
        }

        while (this.actionQueue.length > 0) {
          const action = this.actionQueue.pop();
          if (action !== undefined) {
            switch (action.type) {
              case "resolution": {
                module.setResolution(action.width, action.height);
                break;
              }
              case "texture-data": {
                module.textureData(action.id, action.data);
                break;
              }
              case "load-textures": {
                module.loadTextures();
                break;
              }
              case "set-editor-mode": {
                module.setEditorMode(action.mode);
                break;
              }
              case "set-solid-editor-mode": {
                module.setSolidEditorMode(action.mode);
                break;
              }
              case "set-gizmo": {
                module.setGizmo(action.gizmo);
                break;
              }
              case "select-texture": {
                module.selectTexture(action.id);
                break;
              }
              case "select-prop": {
                module.selectProp(action.id);
                break;
              }
              case "save-scene": {
                module.saveScene();
                break;
              }
            }
          }
        }

        const savedScene = module.getSavedScene();
        if (savedScene !== undefined) {
          this.savedScene = savedScene;
        }
      }, 16);

      module.main();
    });
  }

  setResolution(width: number, height: number) {
    this.actionQueue.unshift([
      {
        type: "resolution",
        width: width,
        height: height,
      },
    ]);
  }

  textureData(id: number, url: string) {
    let get = async () => {
      let image = await fetch(url);
      let arrayBuffer = await image.arrayBuffer();
      let data = new Uint8Array(arrayBuffer);
      this.actionQueue.unshift({
        type: "texture-data",
        id: id,
        data: data,
      });
    };
    get();
  }

  loadTextures() {
    this.actionQueue.unshift({
      type: "load-textures",
    });
  }

  setEditorMode(mode: number) {
    this.actionQueue.unshift({
      type: "set-editor-mode",
      mode: mode,
    });
  }

  setSolidEditorMode(mode: number) {
    this.actionQueue.unshift({
      type: "set-solid-editor-mode",
      mode: mode,
    });
  }

  setGizmo(gizmo: number) {
    this.actionQueue.unshift({
      type: "set-gizmo",
      gizmo: gizmo,
    });
  }

  saveScene() {
    this.actionQueue.unshift({
      type: "save-scene",
    });
  }

  selectTexture(id: number) {
    this.actionQueue.unshift({
      type: "select-texture",
      id: id,
    });
  }

  selectProp(id: number) {
    this.actionQueue.unshift({
      type: "select-prop",
      id: id,
    });
  }

  getSavedScene(): Uint8Array | undefined {
    return this.savedScene;
  }

  destroy() {
    if (this.loopTimeout !== undefined) {
      clearInterval(this.loopTimeout);
    }
  }
}
