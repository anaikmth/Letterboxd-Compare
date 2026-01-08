use std::fs::{self, create_dir, File};
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::PathBuf;

/// Simple file backed cache
pub struct FileCache {}

impl FileCache {
    const CACHE_DIR: &'static str = "./cache";

    pub fn new() -> Result<Self, std::io::Error> {
        match create_dir(Self::CACHE_DIR) {
            Ok(()) => {}
            Err(e) if e.kind() == ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e),
        }
        Ok(FileCache {})
    }

    fn get_cache_file(key: &str) -> PathBuf {
        PathBuf::from(&format!("{}/{}", Self::CACHE_DIR, key))
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, std::io::Error> {
        let mut file = match File::open(Self::get_cache_file(key)) {
            Ok(f) => f,
            Err(e) if e.kind() == ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(err),
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(Some(contents))
    }

    // CORRECTION : Maintenant on écrase toujours le fichier
    pub fn insert(&self, key: &str, value: &str) -> Result<(), std::io::Error> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)      // ← Changé de create_new à create
            .truncate(true)    // ← Ajouté pour vider le fichier avant d'écrire
            .open(Self::get_cache_file(key))?;

        file.write_all(value.as_bytes())
    }
}
