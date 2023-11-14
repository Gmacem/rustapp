use log::{warn, error};
use std::fs;
use std::io::Write;
use std::path::{PathBuf, Path};

use crate::apps::find::args::FindAppArgs;
use crate::controllers::fs::{Fs, FsController};

pub struct FindApp {
    fs: Fs,
    root: PathBuf,
}

impl FindApp {
    pub fn new(root: PathBuf) -> FindApp {
        FindApp {
            fs: Fs {},
            root,
        }
    }

    pub fn run(&self, writer: impl Write, args: &FindAppArgs) -> Result<(), String> {
        let founds = match self.find_files(&args.name) {
            Ok(founds) => founds,
            Err(err) => return Err(err),
        };
        self.print_files(writer, founds)
    }

    fn print_files(&self, mut writer: impl Write, files: Vec<PathBuf>) -> Result<(), String> {
        for file in files {
            let fullpath = fs::canonicalize(file).unwrap_or_default();
            match writer.write(format!("{}\n", fullpath.display()).as_bytes()) {
                Ok(_) => (),
                Err(err) => {
                    error!("Failed to print a file: {}", err);
                    return Err(err.to_string());
                }
            };
        }
        Ok(())
    }

    fn find_files(&self, filename: &str) -> Result<Vec<PathBuf>, String> {
        self.find_file_recursive(filename, &self.root)
    }

    fn find_file_recursive(
        &self,
        filename: &str,
        current_dir: &Path,
    ) -> Result<Vec<PathBuf>, String> {
        let files_and_dirs = match self.fs.get_list_dir(current_dir) {
            Ok(files_and_dirs) => files_and_dirs,
            Err(err) => {
                warn!(
                    "failed to get files in a directory {} by a reason: {}",
                    current_dir.display(),
                    err
                );
                return Err(err);
            }
        };

        let mut result = Vec::new();

        for file in files_and_dirs {
            let name = file
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default();
            if self.fs.is_dir(file.as_path()) {
                if let Ok(founds) = self.find_file_recursive(filename, &file.to_path_buf()) {
                    result.extend(founds);
                }
            }
            if name == filename {
                result.push(file);
            }
        }

        Ok(result)
    }
}
