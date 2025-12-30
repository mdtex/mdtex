// use argh::FromArgs;

pub mod cli;
mod project_management;
mod utils;
mod watcher;

use std::fs;

use cli::MDTeXTopLevelCLI;
use mdtexlib::compile;

use crate::{project_management::project::Project, utils::query::CTAN_URL, watcher::watch};

pub fn run(args: MDTeXTopLevelCLI) {
    match args.nested {
        cli::MDTeXCommands::Compile(compile_sub_command) => {
            compile_sub_command.validate().unwrap_or_else(|e| {
                eprintln!("error: {e}");
                std::process::exit(1);
            });

            if let Some(output) = compile_sub_command.output {
                compile(compile_sub_command.input, output).unwrap_or_else(|e| {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                });
            } else {
                compile(compile_sub_command.input, "./out.html").unwrap_or_else(|e| {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                });
            }
        }
        cli::MDTeXCommands::Watch(watch_sub_command) => watch(watch_sub_command).unwrap(),
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
        cli::MDTeXCommands::Build(_b) => {
            if let Ok(p) = Project::from_directory(std::env::current_dir().unwrap()) {
                let dir = p.directory();

                for input in dir.iter() {
                    if input.extension().is_some_and(|ext| ext == "mdt") {
                        let mut output = input.clone();
                        output.set_extension("html");
                        if let Err(e) = compile(input, output) {
                            eprintln!(
                                "Error while generating output for {}: {}",
                                input.display(),
                                e
                            );
                        }
                    }
                }
            } else {
                eprintln!("Must be in an MDTeX project")
            }
        }
    }
}

#[cfg(test)]
mod tests {}
