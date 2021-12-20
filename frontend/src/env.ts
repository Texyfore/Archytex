interface Environment{
    base_url: string,
    asset_url: string
}

const dev: Environment = {
    base_url: "http://localhost:8080/api/",
    asset_url: "/assets"
}

const prod: Environment = {
    base_url: "TODO",
    asset_url: "/assets"
};

const Environment = process.env.STAGE == 'production' ? prod : dev;

export default Environment;