use std::fs;
use std::fs::metadata;
use std::path::{Path, PathBuf};

use log::warn;

pub trait FsController {
    fn get_list_dir(&self, path: &Path) -> Result<Vec<PathBuf>, String>;
    fn is_file(&self, path: &Path) -> bool;
    fn is_dir(&self, path: &Path) -> bool;
}

pub struct Fs {}

impl FsController for Fs {
    fn get_list_dir(&self, path: &Path) -> Result<Vec<PathBuf>, String> {
        let paths = match fs::read_dir(path) {
            Ok(fc) => fc,
            Err(err) => {
                warn!("Failed to get list of files in a folder");
                return Err(err.to_string());
            }
        };
        let mut result: Vec<PathBuf> = Vec::new();
        for path in paths {
            match path {
                Ok(entry) => result.push(entry.path()),
                Err(err) => return Err(err.to_string()),
            }
        }
        Ok(result)
    }

    fn is_file(&self, path: &Path) -> bool {
        match metadata(path) {
            Ok(md) => md.is_file(),
            Err(_) => false,
        }
    }

    fn is_dir(&self, path: &Path) -> bool {
        match metadata(path) {
            Ok(md) => md.is_dir(),
            Err(_) => false,
        }
    }
}
