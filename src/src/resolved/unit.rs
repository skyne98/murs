use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    pub name: String,
    pub src: String,
}
