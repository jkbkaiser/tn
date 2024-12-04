use miette::{IntoDiagnostic, Result};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

pub struct Cache {
    path: PathBuf,
    files: HashMap<PathBuf, String>,
}

impl Cache {
    pub fn new(path: PathBuf) -> Result<Self> {
        if !path.is_dir() {
            fs::create_dir_all(&path).into_diagnostic()?;
        }

        Ok(Self {
            path,
            files: HashMap::new(),
        })
    }

    pub fn files(&self) -> Vec<PathBuf> {
        self.files.keys().cloned().collect()
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn update<P: AsRef<Path>>(&mut self, path: P) {
        if let Ok(bytes) = std::fs::read(path.as_ref()) {
            let hash = sha256::digest(&bytes);
            self.files.insert(path.as_ref().to_path_buf(), hash);
        }
    }

    pub fn modified<P: AsRef<Path>>(&self, path: P) -> bool {
        match self.files.get(path.as_ref()) {
            Some(hash) => {
                let bytes = std::fs::read(&path).expect("Cache could not read file");
                let file_hash = sha256::digest(&bytes);
                hash != &file_hash
            }
            None => true,
        }
    }
}
