use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Library {
    modules: Vec<PathBuf>,
    links: Vec<String>,
}

impl Library {}
