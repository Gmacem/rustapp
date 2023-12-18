use log::{error, warn};
use std::fs;
use std::io::{Write};
use std::path::{Path, PathBuf};

use crate::controllers::fs::{Fs, FsController};
use crate::utils::sort;

use std::fs::OpenOptions;

#[derive(PartialEq, Eq, Debug)]
pub enum OccurenceKind {
    File,
    Dir,
    TextFile,
}

pub struct Occurence {
    pub path: PathBuf,
    pub kind: OccurenceKind,
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
                let cur_filename = file.as_path().to_str().unwrap();
                if self.fs.is_dir(file.as_path()) {
                    result.push(Occurence {
                        path: file,
                        kind: OccurenceKind::Dir,
                    })
                } else if cur_filename.ends_with(".txt") {
                    result.push(Occurence {
                        path: file,
                        kind: OccurenceKind::TextFile,
                    });
                } else {
                    result.push(Occurence {
                        path: file,
                        kind: OccurenceKind::File,
                    });
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
        sort::sort(&mut context.files, &|a, b| a.path < b.path);
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
            let fullpath = fs::canonicalize(file.path.clone()).unwrap_or_default();
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
            let fullpath = fs::canonicalize(file.path.clone()).unwrap_or_default();
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
          if occurence.kind != OccurenceKind::TextFile {
              return false;
          }

          match fs::read_to_string(&occurence.path) {
              Ok(data) => data.contains(&self.content),
              Err(err) => {
                  warn!("Failed to read file {}: {}", occurence.path.display(), err);
                  false
              }
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
