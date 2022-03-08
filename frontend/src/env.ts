interface Environment {
  base_url: string;
  ws_url: string;
  asset_url: string;
  asset_repo_url: string,
}

const dev: Environment = {
  base_url: "http://localhost:8080/api/",
  ws_url: "ws://localhost:8080/api/ws",
  asset_url: "/assets",
  asset_repo_url: "/assets/db.json",
};

const prod: Environment = {
  base_url: "/api/",
  ws_url: `${window.location.protocol === "https:" ? "wss" : "ws"}://${window.location.host}/api/ws`,
  asset_url: "/assets",
  asset_repo_url: "/assets/repo.json",
};

const Environment = process.env.REACT_APP_STAGE === "production" ? prod : dev;

export default Environment;
