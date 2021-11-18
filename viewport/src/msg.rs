use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum Message {
    AddTexture { uuid: u64, data: String },
}
