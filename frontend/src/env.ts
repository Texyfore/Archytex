interface Environment {
  base_url: string;
  ws_url: string;
  asset_url: string;
  asset_repo_url: string;
  captcha: string;
}

const dev: Environment = {
  base_url: "http://localhost:8080/api/",
  ws_url: "ws://localhost:8080/api/ws",
  asset_url: "/assets",
  asset_repo_url: "/assets/repo.json",
  captcha: '6Lc5gWodAAAAAEVg3MPTn5Nj7KN-ishnafqV4ZL8',
};

const prod: Environment = {
  base_url: "/api/",
  ws_url: `${window.location.protocol === "https:" ? "wss" : "ws"}://${
    window.location.host
  }/api/ws`,
  asset_url: "/assets",
  asset_repo_url: "/assets/repo.json",
  captcha: process.env.REACT_CAPTCHA as string,
};

const Environment = process.env.REACT_APP_STAGE === "production" ? prod : dev;

export default Environment;
