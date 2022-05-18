use serde::Serialize;

#[derive(Serialize)]
pub struct Ping {
    pub msg: String
}

