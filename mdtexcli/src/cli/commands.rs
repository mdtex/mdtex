use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Usage: mdtex compile <flags> <input>
#[argh(subcommand, name = "compile")]
pub(crate) struct CompileSubCommand {
    /// recursively compile files in a given folder
    #[argh(switch, short = 'r')]
    pub(crate) recursive: bool,

    /// specify we want our output as html file(s)
    #[argh(switch)]
    pub(crate) html: bool,

    /// specify we want our output as pdf file(s)
    #[argh(switch)]
    pub(crate) pdf: bool,

    /// specify output to a specific file - must be either pdf or html file extension
    #[argh(option, short = 'o')]
    pub(crate) output: Option<String>,

    /// write output files recursively into a folder
    #[argh(option, long="write-recursive", short = 'w')]
    pub(crate) write_recursive: Option<String>,

    // (maybe): parse input into file or folder
    /// input file/folder
    #[argh(positional)]
    pub(crate) input: String,
}


impl CompileSubCommand {
    /// Validates logical consistency of flags and options
    pub fn validate(&self) -> Result<(), String> {
        // --- 1. Disallow both --html and --pdf ---
        if self.html && self.pdf {
            return Err(String::from("Cannot specify both --html and --pdf at the same time."));
        }

        // --- 2. Validate output extension if provided ---
        if let Some(ref output) = self.output {
            if !(output.ends_with(".html") || output.ends_with(".pdf")) {
                return Err(String::from(
                    "Output file must have either a .html or .pdf extension.",
                ));
            }

            // Optional consistency check
            if self.html && !output.ends_with(".html") {
                return Err(String::from(
                    "Output file extension (.pdf) does not match the --html flag.",
                ));
            }
            if self.pdf && !output.ends_with(".pdf") {
                return Err(String::from(
                    "Output file extension (.html) does not match the --pdf flag.",
                ));
            }
        }

        Ok(())
    }
}


#[derive(FromArgs, PartialEq, Debug)]
/// Usage: mdtex watch <flags> <input>
#[argh(subcommand, name = "watch")]
pub(crate) struct WatchSubCommand {
    // (maybe): parse input into file or folder
    /// input file/folder
    #[argh(positional)]
    pub(crate) input: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Usage: mdtex init <flags> <project-name>
#[argh(subcommand, name = "init")]
pub(crate) struct InitSubCommand {
    // parse input into folder?
    #[argh(positional)]
    pub(crate) project_name: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Usage: mdtex add <flags> <package-name>
#[argh(subcommand, name = "add")]
pub(crate) struct AddSubCommand {
    #[argh(positional)]
    pub(crate) add: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Usage: mdtex build <flags>
#[argh(subcommand, name = "build")]
pub(crate) struct BuildSubCommand {
    #[argh(positional)]
    pub(crate) build: String,
}


/// Unified enum for all possible mdtex subcommands
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub(crate) enum SubCommand {
    Compile(CompileSubCommand),
    Watch(WatchSubCommand),
    Init(InitSubCommand),
    Add(AddSubCommand),
    Build(BuildSubCommand),
}

/// Top-level mdtex command
#[derive(FromArgs, PartialEq, Debug)]
/// A Markdown-to-TeX converter CLI
pub(crate) struct MdtexCommand {
    #[argh(subcommand)]
    pub(crate) subcommand: SubCommand,
}
