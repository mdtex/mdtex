use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;
use std::thread;
use std::{env, time::Duration};

use anyhow::{Context, Error, Result};
use mdtexlib::compile;

use crate::cli::commands::WatchSubCommand;

// since anyhow is included as a dependency, i have written the code in a way i think best
// leverages this fact, preferring anyhow::Error over Box<dyn Error>, as while both are in the end
// a Box<dyn Error>, the former lets us attach context to the error

pub fn watch(args: WatchSubCommand) -> Result<(), Error> {
    let dir = env::current_dir()
        .context("Unable to get current directory. Please check your file permissions")?;

    // load all files into some kind of associative data structure to keep track of delta from last
    // modified time, if greater than certain amount, call compile

    // new mariano here, here's why that sucks. We have to query the fs to check for new files
    // anyways, and to update that last modified time, so there is no work saved by keeping track
    // of that time when we can just produce it as we go for less space

    let mut initial = true;

    loop {
        for entry in read_dir(&dir).context("Unable to read current directory")? {
            if let Ok(handle) = entry {
                let mut dir_entry_path = handle.path();
                let metadata = handle.metadata().context("Unable to file metadata")?;

                // the second clause to this expression is very idiomatic from what i can tell
                if metadata.is_file() && dir_entry_path.extension().is_some_and(|ext| ext == "mdt")
                {
                    // the question is, is this an error that is worth aborting over, or not?
                    let last_modified = metadata
                        .modified()
                        .context("Unable to get last modified time for a file")?;
                    let time_since = last_modified
                        .elapsed()
                        .context("Unable to get time since last modification")?;
                    if initial {
                        println!("Compiling {}", dir_entry_path.display());

                        let contents =
                            read_to_string(&dir_entry_path).context("Unable to read file")?;

                        dir_entry_path.set_extension("html");

                        compile(contents, dir_entry_path)
                            .context("Unable to compile MDTeX file")?;
                    } else if time_since < Duration::from_secs(5) {
                        println!(
                            "{} has been modified, recompiling...",
                            dir_entry_path.display()
                        );

                        let contents =
                            read_to_string(&dir_entry_path).context("Unable to read file")?;

                        dir_entry_path.set_extension("html");

                        compile(contents, dir_entry_path)
                            .context("Unable to compile MDTeX file")?;
                    }
                }
            }
        }

        if initial {
            initial = false;
        }

        thread::sleep(Duration::from_secs(5));
    }
}
