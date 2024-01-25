use log::{error, warn};
use std::fs;
use std::io::{Write};
use std::path::{Path, PathBuf};

use crate::controllers::fs::{Fs, FsController};
use crate::utils::sort;

use std::fs::OpenOptions;

pub struct OccurenceData {
    pub path: PathBuf,
}

pub enum Occurence {
    File(OccurenceData),
    Dir(OccurenceData),
    TextFile(OccurenceData),
}

fn get_occurence_path(occ: &Occurence) -> &PathBuf {
    match occ {
        Occurence::File(data) => &data.path,
        Occurence::Dir(data) => &data.path,
        Occurence::TextFile(data) => &data.path
    }
}

pub struct Context {
    pub name: String,
    pub files: Vec<Occurence>,
}

pub trait ProcessStrategy {
    fn process(&mut self, context: &mut Context) -> Result<(), String>;
}

pub trait PostProcessStrategy {
    fn post_process(&mut self, context: &mut Context) -> Result<(), String>;
}

pub trait PrintStrategy {
    fn print(&mut self, context: &mut Context) -> Result<(), String>;
}

pub struct FindProcess {
    fs: Fs,
    root: PathBuf,
}

impl FindProcess {
    pub fn new(root: PathBuf) -> FindProcess {
        FindProcess { fs: Fs {}, root }
    }

    fn find_files(&self, filename: &str) -> Result<Vec<Occurence>, String> {
        self.find_file_recursive(filename, &self.root)
    }

    fn find_file_recursive(
        &self,
        filename: &str,
        current_dir: &Path,
    ) -> Result<Vec<Occurence>, String> {
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
                let cur_filename: &str = file.as_path().to_str().unwrap();
                if self.fs.is_dir(file.as_path()) {
                    result.push(Occurence::Dir(OccurenceData{path: file}));
                } else if cur_filename.ends_with(".txt") {
                    result.push(Occurence::TextFile(OccurenceData{path: file}));
                } else {
                    result.push(Occurence::File(OccurenceData{path: file}));
                }
            }
        }

        Ok(result)
    }
}

impl ProcessStrategy for FindProcess {
    fn process(&mut self, context: &mut Context) -> Result<(), String> {
        context.files = match self.find_files(&context.name) {
            Ok(founds) => founds,
            Err(err) => return Err(err),
        };
        Ok(())
    }
}

pub struct SortStrategy {}

impl PostProcessStrategy for SortStrategy {
    fn post_process(&mut self, context: &mut Context) -> Result<(), String> {
        sort::sort(&mut context.files, &|a, b|
            get_occurence_path(a) < get_occurence_path(b)
        );
        Ok(())
    }
}

pub struct PrintFileStrategy {
    filename: PathBuf,
}

impl PrintFileStrategy {
    pub fn new(filename: PathBuf) -> PrintFileStrategy {
        PrintFileStrategy { filename }
    }
}

impl PrintStrategy for PrintFileStrategy {
    fn print(&mut self, context: &mut Context) -> Result<(), String> {
        let mut out = match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(self.filename.clone())
        {
            Ok(file) => file,
            Err(err) => return Err(err.to_string()),
        };

        context.files.iter().for_each(move |file| {
            let path = get_occurence_path(file);
            let fullpath = fs::canonicalize(path).unwrap_or_default();
            match writeln!(out, "{}", fullpath.display()) {
                Ok(_) => (),
                Err(err) => {
                    error!("Failed to print a file: {}", err);
                }
            };
        });
        Ok(())
    }
}

pub struct PrintConsoleStrategy {}

impl PrintStrategy for PrintConsoleStrategy {
    fn print(&mut self, context: &mut Context) -> Result<(), String> {
        context.files.iter().for_each(move |file| {
            let fullpath = fs::canonicalize(get_occurence_path(file)).unwrap_or_default();
            println!("{}", fullpath.display());
        });
        Ok(())
    }
}

pub struct InTextFileFilter {
  content: String,
}

impl PostProcessStrategy for InTextFileFilter {
  fn post_process(&mut self, context: &mut Context) -> Result<(), String> {
      context.files.retain(|occurence| {
          if let Occurence::TextFile(data) = occurence {
            match fs::read_to_string(&data.path) {
                Ok(data) => data.contains(&self.content),
                Err(err) => {
                    warn!("Failed to read file {}: {}", data.path.display(), err);
                    false
                }
            }
          } else {
            return false;
          }
      });

      Ok(())
  }
}

impl InTextFileFilter {
  pub fn new(content: String) -> InTextFileFilter {
    InTextFileFilter{
      content,
    }
  }
}

pub struct MultiplePostProcess {
  strategies: Vec<Box<dyn PostProcessStrategy>>,
}

impl PostProcessStrategy for MultiplePostProcess {
  fn post_process(&mut self, context: &mut Context) -> Result<(), String> {
      for strategy in &mut self.strategies {
          match strategy.post_process(context) {
            Ok(()) => (),
            Err(err) => return Err(err),
          }
      }

      Ok(())
  }
}

impl MultiplePostProcess {
  pub fn new() -> MultiplePostProcess {
    MultiplePostProcess{
      strategies: Vec::new(),
    }
  }

  pub fn add_strategy(&mut self, strategy: Box<dyn PostProcessStrategy>) {
    self.strategies.push(strategy)
  }
}
