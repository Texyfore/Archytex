interface Environment{
    base_url: string
}

const dev: Environment = {
    base_url: "http://localhost:8080/api/"
}

const prod: Environment = {
    base_url: "TODO"
};

const Environment = process.env.STAGE == 'production' ? prod : dev;

export default Environment;