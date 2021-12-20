export default class EditorHandle {
  private loopTimeout: NodeJS.Timeout | undefined;
  private currentResolution: [number, number];
  private desiredResolution: [number, number] | undefined;

  constructor() {
    this.loopTimeout = undefined;
    this.currentResolution = [1024, 768];
    this.desiredResolution = undefined;

    import("viewport").then((module) => {
      this.loopTimeout = setInterval(this.loop(module), 16);
      module.main();
    });
  }

  setResolution(width: number, height: number) {
    this.desiredResolution = [width + 1, height];
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
        console.log("res change");
      }
    };
  }
}
