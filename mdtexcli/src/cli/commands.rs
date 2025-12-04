use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Usage: mdtex compile <flags> <input>
#[argh(subcommand, name = "compile")]
pub(crate) struct CompileSubCommand {
    /// recursively compile files in a given folder
    #[argh(switch, short = 'r')]
    recursive: bool,

    /// specify we want our output as html file(s)
    #[argh(switch)]
    html: bool,

    /// specify we want our output as pdf file(s)
    #[argh(switch)]
    pdf: bool,

    /// specify output to a specific file - must be either pdf or html file extension
    #[argh(option, short = 'o')]
    output: Option<String>,

    /// write output files recursively into a folder
    #[argh(option, long="write-recursive", short = 'w')]
    write_recursive: Option<String>,

    /// input file/folder
    #[argh(positional)]
    input: String,
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

        if !self.input.ends_with(".mdt") {
            return Err(String::from(
                "Input file is not .mdt",
            ));
        }

        Ok(())
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Usage: mdtex watch <flags> <input>
#[argh(subcommand, name = "watch")]
pub(crate) struct WatchSubCommand { }

#[derive(FromArgs, PartialEq, Debug)]
/// Usage: mdtex new <flags> <project-name>
#[argh(subcommand, name = "new")]
pub(crate) struct NewSubCommand {
    // parse input into folder?
    #[argh(positional)]
    pub(crate) project_directory: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Usage: mdtex init <flags>
#[argh(subcommand, name = "init")]
pub(crate) struct InitSubCommand { }

#[derive(FromArgs, PartialEq, Debug)]
/// Usage: mdtex add <flags> <package-name>
#[argh(subcommand, name = "add")]
pub(crate) struct AddSubCommand {
    #[argh(positional, greedy)]
    pub(crate) add: Vec<String>,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Usage: mdtex build <flags>
#[argh(subcommand, name = "build")]
pub(crate) struct BuildSubCommand { }

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use crate::*;

    #[test]
    fn test_cli() {
        let mut cmd = Command::cargo_bin("mdtexcli").unwrap();

        cmd.assert().failure();
        cmd.arg("help").assert().success();
    }

    // #[test]
    // fn test_compile() {
    //     // test just compile = failure
    //     let mut compile_cmd = Command::cargo_bin("mdtexcli").unwrap();
    //     compile_cmd.arg("compile");
    //     compile_cmd.assert().failure();

    //     // test compile with html switch
    //     let mut compile_cmd = Command::cargo_bin("mdtexcli").unwrap();
    //     compile_cmd
    //         .args([
    //             "compile",
    //             "--html",
    //             "file.mdt"
    //         ]);
    //     compile_cmd.assert().success();

    //     // test -r switch
    //     let mut compile_cmd = Command::cargo_bin("mdtexcli").unwrap();
    //     compile_cmd
    //         .args([
    //             "compile",
    //             "-r",
    //             "file.mdt"
    //         ]);
    //     compile_cmd.assert().success();

    //     // test pdf switch
    //     let mut compile_cmd = Command::cargo_bin("mdtexcli").unwrap();
    //     compile_cmd
    //         .args([
    //             "compile",
    //             "--pdf",
    //             "file.mdt"
    //         ]);
    //     compile_cmd.assert().success();

    //     // test html + pdf switch = failure
    //     let mut compile_cmd = Command::cargo_bin("mdtexcli").unwrap();
    //     compile_cmd
    //         .args([
    //             "compile",
    //             "--html",
    //             "--pdf",
    //             "file.mdt"
    //         ]);
    //     compile_cmd.assert().failure();

    //     // test output html file
    //     let mut compile_cmd = Command::cargo_bin("mdtexcli").unwrap();
    //     compile_cmd
    //         .args([
    //             "compile",
    //             "--html",
    //             "--output",
    //             "out.html",
    //             "file.mdt"
    //         ]);
    //     compile_cmd.assert().success();

    //     // test output pdf file
    //     let mut compile_cmd = Command::cargo_bin("mdtexcli").unwrap();
    //     compile_cmd
    //         .args([
    //             "compile",
    //             "--pdf",
    //             "--output",
    //             "out.pdf",
    //             "file.mdt"
    //         ]);
    //     compile_cmd.assert().success();

    //     // test output html file with pdf switch = failure
    //     let mut compile_cmd = Command::cargo_bin("mdtexcli").unwrap();
    //     compile_cmd
    //         .args([
    //             "compile",
    //             "--pdf",
    //             "--output",
    //             "out.html",
    //             "file.mdt"
    //         ]);
    //     compile_cmd.assert().failure();

    //     // test output pdf file with html switch = failure
    //     let mut compile_cmd = Command::cargo_bin("mdtexcli").unwrap();
    //     compile_cmd
    //         .args([
    //             "compile",
    //             "--html",
    //             "-o",
    //             "out.pdf",
    //             "file.mdt"
    //         ]);
    //     compile_cmd.assert().failure();
    // }

    // #[test]
    // fn test_watch() {
    //     let mut compile_cmd = Command::cargo_bin("mdtexcli").unwrap();

    //     compile_cmd.arg("watch").assert().success();
    // }

    // #[test]
    // fn test_new() {
    //     let mut compile_cmd = Command::cargo_bin("mdtexcli").unwrap();

    //     compile_cmd.arg("new").assert().failure();
    //     compile_cmd.arg("proj").assert().success();
    // }

    // #[test]
    // fn test_init() {
    //     let mut compile_cmd = Command::cargo_bin("mdtexcli").unwrap();

    //     compile_cmd.arg("init").assert().success();
    // }

    // #[test]
    // fn test_add() {
    //     let mut compile_cmd = Command::cargo_bin("mdtexcli").unwrap();

    //     compile_cmd.arg("add").assert().failure();
    //     compile_cmd.args(["one", "two", "three"]).assert().success();
    // }

    // #[test]
    // fn test_build() {
    //     let mut compile_cmd = Command::cargo_bin("mdtexcli").unwrap();

    //     compile_cmd.arg("build").assert().success();
    // }
}
