use std::io::prelude::*;
use std::path::Path;
use thiserror::Error;

use crate::repo::Repository;

#[derive(Error, Debug)]
pub enum LocalRepositoryError {
    #[error("File not found: {path:?}")]
    FileNotFound { path: String },
    #[error("Not a file: {path:?}")]
    NotAFile { path: String },
    #[error("Not a directory: {path:?}")]
    NotADirectory { path: String },
    #[error("Failed to read file contents")]
    FileReadError(#[from] std::io::Error),
}

pub struct LocalRepository<'src> {
    directory: &'src Path,
}

impl<'src> LocalRepository<'src> {
    pub fn new(path: &'src str) -> Result<Self, LocalRepositoryError> {
        let directory = Path::new(path);
        if !directory.exists() {
            return Err(LocalRepositoryError::FileNotFound {
                path: directory.to_string_lossy().into(),
            });
        }

        if !directory.is_dir() {
            return Err(LocalRepositoryError::NotADirectory {
                path: directory.to_string_lossy().into(),
            });
        }

        Ok(Self { directory })
    }
}

impl<'src> Repository<LocalRepositoryError> for LocalRepository<'src> {
    fn fetch_file_contents(&self, path: &str) -> Result<String, LocalRepositoryError> {
        let full_path = self.directory.join(path);
        if !full_path.exists() {
            Err(LocalRepositoryError::FileNotFound {
                path: full_path.to_string_lossy().into(),
            })?;
        }

        if !full_path.is_file() {
            return Err(LocalRepositoryError::NotAFile {
                path: full_path.to_string_lossy().into(),
            });
        }

        let mut file = std::fs::File::open(full_path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(buffer)
    }
}
