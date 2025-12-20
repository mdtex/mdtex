// use argh::FromArgs;

pub mod cli;
mod project_management;
mod utils;
mod watcher;

use std::fs;

use cli::MDTeXTopLevelCLI;
use mdtexlib::compile;

use crate::{project_management::project::Project, utils::query::CTAN_URL};

pub fn run(args: MDTeXTopLevelCLI) {
    match args.nested {
        cli::MDTeXCommands::Compile(compile_sub_command) => {
            compile_sub_command.validate().unwrap_or_else(|e| {
                eprintln!("error: {e}");
                std::process::exit(1);
            });

            let Ok(text) = fs::read_to_string(&compile_sub_command.input) else {
                eprintln!("error: file '{}' not found", &compile_sub_command.input);
                std::process::exit(1);
            };

            if let Some(output) = compile_sub_command.output {
                compile(text, output).unwrap_or_else(|e| {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                });
            } else {
                compile(text, "./out.html").unwrap_or_else(|e| {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                });
            }
        }
        cli::MDTeXCommands::Watch(watch_sub_command) => {}
        cli::MDTeXCommands::Init(_) => {
            let mut proj = Project::init(std::env::current_dir().unwrap());
            let result = proj.create_project();

            if result.is_ok() {
                println!("Successfully created project");
            } else {
                println!("Could not create project");
            }
        }
        cli::MDTeXCommands::New(new_sub_command) => {
            let mut proj = Project::new(new_sub_command.project_directory);
            let result = proj.create_project();

            if result.is_ok() {
                println!("Successfully created project {}", proj.name);
            } else {
                println!("Could not create project");
            }
        }
        cli::MDTeXCommands::Add(add_sub_command) => {
            #[allow(unused_mut)]
            let mut proj = Project::from_directory(std::env::current_dir().unwrap());

            if proj.is_err() {
                panic!("Project not found!");
            }

            let result = proj.unwrap().add_packages(add_sub_command.add, CTAN_URL);

            if result.is_ok() {
                let mut packages_added = result.unwrap();

                if packages_added.is_empty() {
                    println!("No packages successfully added");
                } else {
                    print!("Successfully added: ");
                    while !packages_added.is_empty() {
                        let package = packages_added.pop().unwrap();

                        if packages_added.is_empty() {
                            println!("{}", package);
                        } else {
                            print!("{}, ", package);
                        }
                    }
                }
            } else {
                panic!("Error adding packages: {:?}", result);
            }
        }
        cli::MDTeXCommands::Build(_) => {}
    }
}

#[cfg(test)]
mod tests {}
