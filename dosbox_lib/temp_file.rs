use std::fs;
use std::path::{Path, PathBuf};

pub struct TempFile(PathBuf);

impl Drop for TempFile {
    fn drop(&mut self) {
        fs::remove_file(&mut self.0).unwrap();
    }
}

impl From<PathBuf> for TempFile {
    fn from(path: PathBuf) -> Self {
        Self(path)
    }
}

impl AsRef<PathBuf> for TempFile {
    fn as_ref(&self) -> &PathBuf {
        &self.0
    }
}

impl AsRef<Path> for TempFile {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl std::ops::Deref for TempFile {
    type Target = PathBuf;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
