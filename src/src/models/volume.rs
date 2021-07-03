use std::path::PathBuf;

pub enum VolumeOrigin {
    Directory { path: PathBuf },
    Git { url: String, branch: Option<String> },
}

pub struct Volume {
    pub origin: VolumeOrigin,
}

impl Volume {
    
}