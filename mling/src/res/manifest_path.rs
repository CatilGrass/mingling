use std::path::PathBuf;

#[derive(Default, Clone)]
pub struct ResManifestPath {
    pub raw: Option<String>,
    pub resolved: Option<PathBuf>,
}

impl ResManifestPath {
    pub fn resolved(&self) -> &PathBuf {
        self.resolved.as_ref().unwrap()
    }
}
