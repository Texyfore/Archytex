use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum Message {
    Dummy(String),
}
