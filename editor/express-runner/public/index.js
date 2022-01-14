import("/editor/web_runner.js").then((module) => {
    console.log(module.WebIpcBackend);
  let channel = module.WebIpcBackend.new();
  let frontend = channel.frontend();
  let backend = channel.backend();
  module.start(backend);
});
