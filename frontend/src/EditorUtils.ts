export default class EditorHandle {
  private module: any | undefined;
  private loopTimeout: NodeJS.Timeout | undefined;

  constructor() {
    this.module = undefined;
    this.loopTimeout = undefined;
    import("viewport").then((module) => {
      this.module = module;
      module.main();
      this.loopTimeout = setInterval(this.loop, 100);
    });
  }

  setResolution(width: number, height: number) {}

  destroy() {
    if (this.loopTimeout !== undefined) {
      clearInterval(this.loopTimeout);
    }
  }

  private loop() {}
}
