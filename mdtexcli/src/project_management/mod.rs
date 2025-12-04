pub(crate) mod project;
pub(crate) mod directory;

use serde::{Serialize, Deserialize};

/// Helper function to return custom error on child file not found in DirectoryWrapper
fn child_not_found(child: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::NotFound, format!("Child file not found: {}", child))
}

/// Helper function to return custom error on project not detected in DirectoryWrapper
fn project_not_found() -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::NotFound, format!("Could not find MDTeX.toml file"))
}

/// LaTeX package structure as returned by CTAN API
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    /// Name of our package
    pub title: String,

    /// The path of our package relative to the CTAN home url
    pub path: String,

    /// The text description of our package
    pub text: Option<String>,
}

/// Custom PackageError type for CTAN queries
#[derive(Debug)]
pub enum PackageError {
    /// Package not found in CTAN error
    NotFound(String),

    /// Wrong package found in CTAN error
    WrongResult(String, String),
}

impl std::fmt::Display for PackageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NotFound(item) => write!(f, "Package not found: {}", item),
            Self::WrongResult(expected, returned) => write!(f, "Found {} instead of {}", returned, expected)
        }
    }
}

impl std::error::Error for PackageError {}