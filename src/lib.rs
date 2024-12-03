use home::home_dir;
use std::path::PathBuf;

pub mod compiler;
pub mod config;
pub mod crawler;
pub mod server;

// TODO make this configurable
pub fn get_storage_dir() -> PathBuf {
    let home = home_dir().expect("Could not read storage directory '~/.tn'");
    home.join(".tn")
}

pub fn get_cache_dir() -> PathBuf {
    let storage_dir = get_storage_dir();
    storage_dir.join("cache")
}

pub fn get_assets_dir() -> PathBuf {
    let storage_dir = get_storage_dir();
    storage_dir.join("assets")
}
