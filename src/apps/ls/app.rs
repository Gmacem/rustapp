use crate::controllers::fs::{Fs, FsController};
use std::io::Write;
use std::{os::unix::prelude::OsStrExt, path::Path, path::PathBuf};

pub struct LsApp {
    fs: Fs,
    root: PathBuf,
}

impl LsApp {
    pub fn new(root: PathBuf) -> LsApp {
        LsApp {
            fs: Fs {},
            root,
        }
    }

    pub fn print_files(&self, mut writer: impl Write, str_path: String) -> Result<(), String> {
        let path_buf = self.get_abs_path_from_str(str_path);
        let path = path_buf.as_path();
        let list_files = match self.get_files_in_dir(path) {
            Ok(list_files) => list_files,
            Err(err) => return Err(err),
        };
        for file in list_files {
            let file_name = file
                .file_name()
                .expect("Failed to get file name")
                .as_bytes();
            match writer.write(file_name) {
                Ok(_) => (),
                Err(err) => return Err(err.to_string()),
            }
            match writer.write(&[b'\n']) {
                Ok(_) => (),
                Err(err) => return Err(err.to_string()),
            }
        }
        Ok(())
    }

    fn get_abs_path_from_str(&self, str_path: String) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(str_path);
        if path.is_relative() {
            return self.root.join(path);
        }
        path
    }

    fn get_files_in_dir(&self, path: &Path) -> Result<Vec<PathBuf>, String> {
        let list_files = match self.fs.get_list_dir(path) {
            Ok(list_dir) => list_dir,
            Err(err) => return Err(err),
        }
        .into_iter()
        .filter(|path| self.fs.is_file(path))
        .collect();
        Ok(list_files)
    }
}
