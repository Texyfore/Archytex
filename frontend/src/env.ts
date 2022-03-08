interface Environment {
  base_url: string;
  ws_url: string;
  asset_url: string;
}

const dev: Environment = {
  base_url: "http://localhost:8080/api/",
  ws_url: "ws://localhost:8080/api/ws",
  asset_url: "/assets",
};

const prod: Environment = {
  base_url: "/api/",
  ws_url: `${window.location.protocol === "https:" ? "wss" : "ws"}://${window.location.host}/api/ws`,
  asset_url: "/assets",
};

const Environment = process.env.REACT_APP_STAGE === "production" ? prod : dev;

export default Environment;
