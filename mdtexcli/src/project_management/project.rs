use crate::{
    project_management::{Package, PackageError, directory::DirectoryWrapper, project_not_found},
    utils::query::{self, CTANReturn},
};
use std::{collections::BTreeMap, error::Error, path::PathBuf};

use serde::{Deserialize, Serialize};

/// Project structure for an MDTeX project
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Project {
    /// The name of our project
    #[serde(rename = "project-name")]
    pub name: String,

    /// The directory wrapper containing our project's files
    #[serde(skip)]
    directory: Box<DirectoryWrapper>,

    /// A list of our projects packages; stored as a BTreeMap for serialization purposes
    #[serde(rename = "packages")]
    package_list: BTreeMap<String, Package>,
    // side note - should probably switch to a vector and then custom serialize/deserialize as a BTreeMap
}

impl Project {
    /// initiate a new project in a given directory path
    pub fn init(directory_path: PathBuf) -> Project {
        let directory = DirectoryWrapper::new(directory_path);

        // parse name from directory path
        let name = directory
            .directory_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let directory = Box::new(directory);

        let package_list = BTreeMap::new();

        Self {
            name,
            directory,
            package_list,
        }
    }

    pub fn directory(&self) -> &Box<DirectoryWrapper> {
        &self.directory
    }

    /// Create a new project from a given project directory - distinct from init for bookkeeping purposes
    pub fn new(project_directory: String) -> Project {
        Self::init(PathBuf::from(project_directory))
    }

    /// Creates this given project and its associated project files
    pub fn create_project(&mut self) -> Result<(), Box<dyn Error>> {
        self.directory.as_mut().create_directory()?;
        self.directory.create_file("MDTeX.toml")?;

        // write the current project metadata to the configuration file
        self.directory
            .write_file("MDTeX.toml", &toml::to_string_pretty(self)?)?;

        Ok(())
    }

    /// Create a Project struct from an existing directory.
    ///
    /// If the directory does not have the necessary configuration files, return a project not found error
    pub fn from_directory(directory: PathBuf) -> Result<Project, Box<dyn Error>> {
        let directory_wrapper = DirectoryWrapper::from(directory)?;

        let find_project_file = directory_wrapper.find_child("MDTeX.toml")?;

        if let Some(project_file) = find_project_file {
            let contents = std::fs::read_to_string(project_file)?;

            let mut project: Project = toml::from_str(&contents)?;
            project.directory = Box::new(directory_wrapper);
            return Ok(project);
        } else {
            return Err(Box::new(project_not_found()));
        }
    }

    /// Add the list of packages passed to our project by querying the CTAN API to check
    /// if they exist, then adding their metadata to our project file
    pub fn add_packages(
        &mut self,
        add: Vec<String>,
        url: &str,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        for package in add {
            let res = self.add_package(package.clone(), url);

            if res.is_ok() {
                let package_added = res.unwrap();
                self.package_list.insert(
                    package_added
                        .title
                        .strip_prefix("Package ")
                        .unwrap_or(&package_added.title)
                        .to_string(),
                    package_added,
                );
            } else {
                // todo: keep track of failed adds maybe?
                println!("Failed to add {}", package);
            }
        }

        // write our package metadata to the toml file
        self.write_metadata("MDTeX.toml")?;

        let added = self.package_list.keys().cloned().collect();

        Ok(added)
    }

    /// Attempt to find a given package. If found, return the package details; else, return an error.
    fn add_package(&mut self, package_name: String, url: &str) -> Result<Package, Box<dyn Error>> {
        let query = query::CTANSend {
            phrase: package_name.clone(),
            max: Some(1),
            ext: Some(true),
            pkg: Some(true),
            ..Default::default()
        };

        let result = CTANReturn::query_ctan_with_base(query, url)?;

        if result.number_of_hits == 0 {
            return Err(Box::new(PackageError::NotFound(package_name)));
        }

        let added = result.hits.first().unwrap();

        if &added.title != &format!("Package {}", package_name) {
            return Err(Box::new(PackageError::WrongResult(
                package_name,
                added.title.clone(),
            )));
        }

        Ok(added.clone())
    }

    /// Write project metadata to the passed file
    pub fn write_metadata(&mut self, file_name: &str) -> Result<(), Box<dyn Error>> {
        let contents_to_write = toml::to_string_pretty(self)?;
        self.directory.write_file(file_name, &contents_to_write)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, path::PathBuf};

    use assert_fs::prelude::*;

    use crate::{
        project_management::{Package, directory::DirectoryWrapper, project::Project},
        utils::query::CTAN_URL,
    };

    const EXPECTED_KNUTH_JSON: &str = r#"
        {
            "numberOfHits": 70,
            "offset": 0,
            "max": 1,
            "phrase": "knuth",
            "hits": [
                {
                    "title": "Package knuth-local",
                    "path": "/pkg/knuth-local",
                    "text": "Knuth’s local information"
                }
            ]
        }
    "#;

    const EXPECTED_PSEUDOCODE_JSON: &str = r#"
        {
            "numberOfHits": 16,
            "offset": 0,
            "max": 1,
            "phrase": "pseudocode",
            "hits": [
                {
                    "title": "Package pseudocode",
                    "path": "/pkg/pseudocode",
                    "text": "LaTeX environment for specifying algorithms in a natural way"
                }
            ]
        }
    "#;

    const EXPECTED_HEP_MATH_JSON: &str = r#"
        {
            "numberOfHits": 204,
            "offset": 0,
            "max": 1,
            "phrase": "math",
            "hits": [
                {
                    "title": "Package hep-math",
                    "path": "/pkg/hep-math",
                    "text": "Extended math macros"
                }
            ]
        }
    "#;

    #[test]
    fn test_create_project() {
        let temp_dir = assert_fs::TempDir::new().unwrap();

        let project_name = "temp_project";

        let dir_path = temp_dir.child(project_name);
        let dir_path_str = dir_path.path().to_string_lossy().into_owned();

        let mut project = Project::new(dir_path_str.clone());

        let expected = Project {
            name: project_name.to_string(),
            directory: Box::new(DirectoryWrapper::new(dir_path.path().to_path_buf())),
            package_list: BTreeMap::new(),
        };

        assert_eq!(project, expected);

        let res = project.create_project();

        assert!(res.is_ok());
        dir_path.assert(predicates::path::exists());

        let toml_file = dir_path.child("MDTeX.toml");
        toml_file.assert("project-name = \"temp_project\"\n\n[packages]\n");
    }

    fn test_write_metadata() {
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let dir = temp_dir.child("project_directory");

        let mut project = Project {
            name: "example project".to_string(),
            directory: Box::new(DirectoryWrapper::new(dir.to_path_buf())),
            package_list: {
                let mut m = BTreeMap::new();
                m.insert(
                    "pseudocode".to_string(),
                    Package {
                        title: "Package pseudocode".to_string(),
                        path: "/pkg/pseudocode".to_string(),
                        text: Some(
                            "LaTeX environment for specifying algorithms in a natural way"
                                .to_string(),
                        ),
                    },
                );
                m
            },
        };

        let res = project.create_project();
        assert!(res.is_ok());

        let res = project.write_metadata("MDTeX.toml");
        assert!(res.is_ok());
    }

    #[test]
    fn test_add_packages_with_one_fail_and_write_metadata() {
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let project_name = "temp_project";
        let dir_path = temp_dir.child(project_name);
        let dir_path_str = dir_path.path().to_string_lossy().into_owned();
        let mut project = Project::new(dir_path_str.clone());
        project.create_project().unwrap();

        let mut server = mockito::Server::new();
        let mut url = server.url();
        url.push_str("/search/json");

        let mock_knuth = server
            .mock("GET", "/search/json")
            .match_query(mockito::Matcher::UrlEncoded(
                "phrase".into(),
                "knuth-local".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(EXPECTED_KNUTH_JSON)
            .create();

        let mock_pseudocode = server
            .mock("GET", "/search/json")
            .match_query(mockito::Matcher::UrlEncoded(
                "phrase".into(),
                "pseudocode".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(EXPECTED_PSEUDOCODE_JSON)
            .create();

        let mock_math = server
            .mock("GET", "/search/json")
            .match_query(mockito::Matcher::UrlEncoded("phrase".into(), "math".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(EXPECTED_HEP_MATH_JSON)
            .create();

        let packages_to_add: Vec<String> = vec![
            "knuth-local".to_string(),
            "pseudocode".to_string(),
            "math".to_string(),
        ];

        project.add_packages(packages_to_add, &url).unwrap();

        assert!(project.package_list.len() == 2);

        let expected = indoc::indoc! { r#"
            project-name = "temp_project"

            [packages.knuth-local]
            title = "Package knuth-local"
            path = "/pkg/knuth-local"
            text = "Knuth’s local information"

            [packages.pseudocode]
            title = "Package pseudocode"
            path = "/pkg/pseudocode"
            text = "LaTeX environment for specifying algorithms in a natural way"
        "# };

        let toml_file = dir_path.child("MDTeX.toml");
        toml_file.assert(expected);
    }

    #[test]
    fn test_from_directory() {
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let directory = temp_dir.child("test_project");
        directory.create_dir_all().unwrap();

        let toml_file = directory.child("MDTeX.toml");
        toml_file.touch().unwrap();

        toml_file.write_str(
            r#"
            project-name="test_project"
        
            [packages.pseudocode]
            title = "Package pseudocode"
            path = "/pkg/pseudocode"
            text = "LaTeX environment for specifying algorithms in a natural way"
        "#,
        );

        let project = Project::from_directory(PathBuf::from(directory.to_path_buf()));

        assert!(project.is_ok());
        let project = project.unwrap();

        assert_eq!(&project.name, "test_project");
        assert_eq!(
            project.package_list["pseudocode"],
            Package {
                title: "Package pseudocode".to_string(),
                path: "/pkg/pseudocode".to_string(),
                text: Some(
                    "LaTeX environment for specifying algorithms in a natural way".to_string()
                ),
            }
        );
    }

    #[test]
    fn test_from_directory_toml_not_found() {
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let directory = temp_dir.child("test_project");
        directory.create_dir_all().unwrap();

        let project = Project::from_directory(PathBuf::from(directory.to_path_buf()));

        assert!(project.is_err());
    }

    #[test]
    fn test_no_result() {
        let mut project = Project::new("project/".to_string());

        let mut server = mockito::Server::new();
        let mut url = server.url();
        url.push_str("/search/json");

        let res = project.add_package("Huh".to_string(), &url);

        assert!(project.package_list.is_empty());
        assert!(res.is_err());

        let mock_err = server
            .mock("GET", "/search/json")
            .match_query(mockito::Matcher::UrlEncoded("phrase".into(), "Huh".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                    "numberOfHits": 0,
                    "offset": 0,
                    "max": 1,
                    "phrase": "huh",
                    "hits": []
                }
            "#,
            )
            .create();

        let res = project.add_package("Huh".to_string(), &url);

        assert!(project.package_list.is_empty());
        assert!(res.is_err());
    }
}

