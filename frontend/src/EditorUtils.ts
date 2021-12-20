class Texture {
  id: number;
  bytes: Uint8Array;
  ptr: number;

  constructor(id: number, bytes: Uint8Array) {
    this.id = id;
    this.bytes = bytes;
    this.ptr = 0;
  }

  eof(): boolean {
    return this.ptr === this.bytes.length;
  }

  next(length: number): Uint8Array {
    let end = Math.min(this.ptr + length, this.bytes.length);
    let arr = this.bytes.subarray(this.ptr, end);

    this.ptr = Math.min(this.ptr + length, this.bytes.length);
    return arr;
  }
}

export default class EditorHandle {
  private loopTimeout: NodeJS.Timeout | undefined;
  private currentResolution: [number, number];
  private desiredResolution: [number, number] | undefined;
  private textures: Texture[];

  constructor() {
    this.loopTimeout = undefined;
    this.currentResolution = [1024, 768];
    this.desiredResolution = undefined;
    this.textures = [];

    import("viewport").then((module) => {
      this.loopTimeout = setInterval(this.loop(module), 16);
      module.main();
    });
  }

  setResolution(width: number, height: number) {
    this.desiredResolution = [width + 1, height];
  }

  loadTexture(id: number, url: string) {
    let get = async () => {
      let image = await fetch(url);
      let arrayBuffer = await image.arrayBuffer();
      let bytes = new Uint8Array(arrayBuffer);
      this.textures.push(new Texture(id, bytes));
    };
    get();
  }

  destroy() {
    if (this.loopTimeout !== undefined) {
      clearInterval(this.loopTimeout);
    }
  }

  private loop(module: any) {
    return () => {
      if (
        this.currentResolution !== this.desiredResolution &&
        this.desiredResolution !== undefined
      ) {
        module.setResolution(
          this.desiredResolution[0],
          this.desiredResolution[1]
        );
        this.currentResolution = this.desiredResolution;
      }

      let texture = this.textures[0];
      if (texture !== undefined) {
        module.sendTextureData(texture.id, texture.next(1024));
        console.log(`${texture.ptr} / ${texture.bytes.length}`);
        if (texture.eof()) {
          module.finishTexture(texture.id);
          this.textures.pop();
        }
      }
    };
  }
}
