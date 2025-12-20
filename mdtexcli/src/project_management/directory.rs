use std::{error::Error, io::Write, path::PathBuf};
use super::child_not_found;

/// DirectoryWrapper struct to keep track of the current directories path and children
#[derive(Debug, Default, PartialEq)]
/// Wrapper for our directory
pub(crate) struct DirectoryWrapper {
    pub(super) directory_path: PathBuf,
    children: Vec<PathBuf>,
}

impl DirectoryWrapper {
    /// Create a new directory wrapper from a soon to be created directory path
    pub fn new(directory_path: PathBuf) -> DirectoryWrapper {
        Self {
            directory_path,
            children: vec![],
        }
    }

    /// Create a new directory wrapper from an already existing directory path
    pub fn from(directory_path: PathBuf) -> Result<DirectoryWrapper, Box<dyn Error>> {
        let mut children: Vec<PathBuf> = Vec::new();
        
        // recursively read all the children in the given directory
        let paths = std::fs::read_dir(&directory_path)?;
        for p in paths {
            let path = p?.path().to_path_buf();

            if path.is_dir() {
                let inner_directory = Self::from(path.clone())?;
                children.extend(inner_directory.children);
            } else {
                children.push(path);
            }
        }

        Ok(
            Self {
                directory_path,
                children,
            }
        )
    }

    /// Creates the direcotry given its path.
    pub fn create_directory(&self) -> Result<(), Box<dyn Error>> {
        std::fs::create_dir_all(&self.directory_path)?;

        Ok(())
    }

    /// Creates a file inside of our directory.
    pub fn create_file(&mut self, file_path_string: &str) -> Result<(), Box<dyn Error>> {
        let mut file_path = self.directory_path.clone();
        file_path.push(file_path_string);

        let _ = std::fs::File::create(&file_path)?;

        self.children.push(file_path);

        Ok(())
    }

    /// Creates a folder inside of our directory
    pub fn create_folder(&mut self, folder_path_string: &str) -> Result<(), Box<dyn Error>> {
        let mut folder_path = self.directory_path.clone();
        folder_path.push(folder_path_string);

        let _ = std::fs::create_dir_all(&folder_path);

        Ok(())
    }

    /// Find a given file in our directory from its file name
    pub fn find_child(&self, file_name: &str) -> Result<Option<&PathBuf>, Box<dyn Error>> {
        let file_path = self.children.iter().find(
            |x| x.file_name().unwrap().to_string_lossy().into_owned() == file_name
        );

        Ok(file_path)
    }

    /// Writes to a file inside of our directory.
    pub fn write_file(&mut self, file_name: &str, contents_to_write: &str) -> Result<(), Box<dyn Error>> {
        let file = self.find_child(file_name)?;

        if let Some(file_path) = file {
            let mut file = std::fs::OpenOptions::new().write(true).open(file_path)?;
            file.write_all(contents_to_write.as_bytes())?;
        } else {
            return Err(Box::new(child_not_found(file_name)));
        }

        Ok(())
    }

    /// Appends to a file inside of our directory.
    pub fn append_file(&mut self, file_name: &str, contents_to_write: &str) -> Result<(), Box<dyn Error>> {
        let file = self.find_child(file_name)?;

        if let Some(file_path) = file {
            let mut file = std::fs::OpenOptions::new().append(true).open(file_path)?;
            file.write_all(contents_to_write.as_bytes())?;
        } else {
            return Err(Box::new(child_not_found(file_name)));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use assert_fs::prelude::*;

    use crate::project_management::directory::DirectoryWrapper;

    #[test]
    fn test_create_directory() {
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let directory_path = temp_dir.child("test_directory");
        let dir = DirectoryWrapper::new(directory_path.path().to_path_buf());

        let res = dir.create_directory();

        assert!(res.is_ok());
    }

    #[test]
    fn test_child_found_on_create() {
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let directory_path = temp_dir.child("test_directory");
        let mut dir = DirectoryWrapper::new(directory_path.path().to_path_buf());
        let _ = dir.create_directory();

        let res = dir.create_file("test.txt");

        assert!(res.is_ok());
        directory_path.child("test.txt").assert(predicates::path::exists());
    }

    #[test]
    fn test_child_not_found_on_write_or_append() {
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let directory_path = temp_dir.child("test_directory");
        let mut dir = DirectoryWrapper::new(directory_path.path().to_path_buf());
        let _ = dir.create_directory();

        let err_res_write = dir.write_file("This shouldn't work", "");
        let err_res_append = dir.append_file("This shouldn't work", "");

        assert!(err_res_write.is_err());
        assert!(err_res_append.is_err());

        directory_path.child("This shouldn't work").assert(predicates::path::missing());
    }

    #[test]
    fn test_create_and_write_child() {
        let temp_dir = assert_fs::TempDir::new().unwrap();

        let directory_path = temp_dir.child("test_directory");
        let mut dir = DirectoryWrapper::new(directory_path.path().to_path_buf());

        let dir_res = dir.create_directory();
        assert!(dir_res.is_ok());

        directory_path.assert(predicates::path::exists());

        let child_res =  dir.create_file("temp.txt");
        assert!(child_res.is_ok());

        let file = directory_path.child("temp.txt");
        file.assert(predicates::path::exists());

        let expected_contents_1 = "Hello, world!";

        let write_res_1 = dir.write_file("temp.txt", "Hello, world!");
        assert!(write_res_1.is_ok());
        file.assert(expected_contents_1);

        let expected_contents_2 = "Hello, world! This is appended.";

        let write_res_2 = dir.append_file("temp.txt", " This is appended.");
        assert!(write_res_2.is_ok());
        file.assert(expected_contents_2);
    }

    #[test]
    fn test_create_directories() {
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let directory_path = temp_dir.child("test_directory");
        let mut dir = DirectoryWrapper::new(directory_path.path().to_path_buf());
        let _ = dir.create_directory();

        let res = dir.create_folder("a/b/c");
        assert!(res.is_ok());
        directory_path.child("a/b/c").assert(predicates::path::exists());
    }

    #[test]
    fn test_create_nested_children() {
        /*
        File Structure:
            test_directory/
                test.txt
                a/
                    a.txt
                    b/
                        b.txt
                        c/
        */

        let temp_dir = assert_fs::TempDir::new().unwrap();
        let directory_path = temp_dir.child("test_directory");
        let mut dir = DirectoryWrapper::new(directory_path.path().to_path_buf());
        let _ = dir.create_directory();

        let res = dir.create_folder("a/b/c");
        assert!(res.is_ok());
        directory_path.child("a/b/c").assert(predicates::path::exists());

        let res = dir.create_file("test.txt");
        assert!(res.is_ok());
        directory_path.child("test.txt").assert(predicates::path::exists());

        let res = dir.create_file("a/a.txt");
        assert!(res.is_ok());
        directory_path.child("a/a.txt").assert(predicates::path::exists());

        let res = dir.create_file("a/b/b.txt");
        assert!(res.is_ok());
        directory_path.child("a/b/b.txt").assert(predicates::path::exists());
    }

    #[test]
    fn test_from() {
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let directory_path = temp_dir.child("test_directory");
        let mut dir = DirectoryWrapper::new(directory_path.path().to_path_buf());
        dir.create_directory().unwrap();

        let res1 = dir.create_file("test.txt");
        let res2= dir.create_folder("test_subdirectory");
        let res3 = dir.create_file("test_subdirectory/test.txt");

        assert!(res1.is_ok() && res2.is_ok() && res3.is_ok());

        let dir_2_res = DirectoryWrapper::from(directory_path.path().to_path_buf().to_path_buf());
        assert!(dir_2_res.is_ok());
        
        let dir_2 = dir_2_res.unwrap();

        assert_eq!(dir.directory_path, dir_2.directory_path);
        assert_eq!(dir.children.len(), dir_2.children.len());
    }

    #[test]
    fn test_from_nested_children() {
        /*
        File Structure:
            test_directory/
                test.txt
                a/
                    a.txt
                    b/
                        b.txt
                        c/
        */

        let temp_dir = assert_fs::TempDir::new().unwrap();
        let directory_path = temp_dir.child("test_directory");
        directory_path.create_dir_all().unwrap();

        // create structure:
        // root/
        //   a.txt
        //   sub/
        //       b.txt
        //       deeper/
        //           c.txt
        let a = directory_path.child("a.txt");
        a.touch().unwrap();

        let sub = directory_path.child("sub");
        sub.create_dir_all().unwrap();
        let b = sub.child("b.txt");
        b.touch().unwrap();

        let deeper = sub.child("deeper");
        deeper.create_dir_all().unwrap();
        let c = deeper.child("c.txt");
        c.touch().unwrap();

        let mut dir = DirectoryWrapper::from(directory_path.to_path_buf()).unwrap();

        assert_eq!(dir.children.len(), 3);
        let paths: Vec<_> = dir.children.iter().map(|p| p.file_name().unwrap().to_string_lossy().to_string()).collect();

        assert!(paths.contains(&"a.txt".to_string()));
        assert!(paths.contains(&"b.txt".to_string()));
        assert!(paths.contains(&"c.txt".to_string()));
    }

    #[test]
    fn test_overrite() {
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let directory_path = temp_dir.child("test_directory");

        let mut dir = DirectoryWrapper::new(directory_path.to_path_buf());
        dir.create_directory().unwrap();
        dir.create_file("data.txt").unwrap();

        let file = directory_path.child("data.txt");

        dir.write_file("data.txt", "hello").unwrap();
        file.assert("hello");

        dir.write_file("data.txt", "new content").unwrap();
        file.assert("new content");
    }

    #[test]
    fn test_append() {
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let directory_path = temp_dir.child("test_directory");

        let mut dir = DirectoryWrapper::new(directory_path.path().to_path_buf());
        dir.create_directory().unwrap();
        dir.create_file("data.txt").unwrap();

        let file = directory_path.child("data.txt");

        dir.write_file("data.txt", "first").unwrap();
        dir.append_file("data.txt", " second").unwrap();

        file.assert("first second");
    }
}