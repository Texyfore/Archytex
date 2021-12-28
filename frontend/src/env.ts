interface Environment{
    base_url: string
    ws_url: string
}

const dev: Environment = {
    base_url: "http://localhost:8080/api/",
    ws_url: "ws://localhost:8080/api/ws",
}

const prod: Environment = {
    base_url: "TODO",
    ws_url: "TODO",
};

const Environment = process.env.STAGE == 'production' ? prod : dev;

export default Environment;