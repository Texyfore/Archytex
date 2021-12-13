import Environment from "../env"

const url = Environment.base_url + "register";
export const Register = async (username: string, password: string, email: string, captcha: string) => {
    const body = {
        username,
        password,
        email,
        captcha
    };
    const resp = await fetch(url, {
        method: 'POST',
        headers:{
            "Content-Type": "application/json"
        },
        body: JSON.stringify(body)
    });
    if (resp.status !== 200) {
        const json = await resp.json();
        throw json;
    }
}