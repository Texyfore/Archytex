use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum Message {
    AddTexture { id: usize, data: Vec<u8> },
}
