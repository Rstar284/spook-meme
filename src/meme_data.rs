use serde::Deserialize;

#[derive(Deserialize)]
pub struct Meme {
    pub id: u32,
    pub creator_id: String,
    pub name: String,
    pub image_url: String,
    pub server_id: String,
    pub parameter: Vec<Parameter>,
}

#[derive(Deserialize)]
pub struct Parameter {
    pub id: u32,
    pub meme_id: u32,
    pub position: Vec<Position>,
}

#[derive(Deserialize)]
pub struct Position {
    pub id: u32,
    pub box_left: u32,
    pub box_top: u32,
    pub box_right: u32,
    pub box_bottom: u32,
    pub parameter_id: u32,
}
