interface Environment {
  base_url: string;
  ws_url: string;
  asset_url: string;
  asset_db_url: string,
}

const dev: Environment = {
  base_url: "http://localhost:8080/api/",
  ws_url: "ws://localhost:8080/api/ws",
  asset_url: "/assets",
  asset_db_url: "/assets/db.json",
};

const prod: Environment = {
  base_url: "TODO",
  ws_url: "TODO",
  asset_url: "TODO",
  asset_db_url: "/assets/db.json",
};

const Environment = process.env.STAGE === "production" ? prod : dev;

export default Environment;
