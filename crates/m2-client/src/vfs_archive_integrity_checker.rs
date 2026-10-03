use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub enum IntegrityError {
    BasePathNotFound,
    BasePathNotDirectory,
    MissingArchive(String),
    ArchiveNotFile(String),
}

pub struct VfsArchiveIntegrityChecker {
    required_archives: Vec<&'static str>,
}

impl VfsArchiveIntegrityChecker {
    pub fn new() -> Self {
        Self {
            required_archives: vec!["root.pck", "locale.pck", "ETC.pck", "uiscript.pck"],
        }
    }

    pub fn check_archives(&self, base_path: &Path) -> Result<(), IntegrityError> {
        if !base_path.exists() {
            return Err(IntegrityError::BasePathNotFound);
        }

        if !base_path.is_dir() {
            return Err(IntegrityError::BasePathNotDirectory);
        }

        for &archive in &self.required_archives {
            let archive_path = base_path.join(archive);

            if !archive_path.exists() {
                return Err(IntegrityError::MissingArchive(archive.to_string()));
            }

            if !archive_path.is_file() {
                return Err(IntegrityError::ArchiveNotFile(archive.to_string()));
            }
        }

        Ok(())
    }
}
