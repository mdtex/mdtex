use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Usage: mdtex compile <flags> <input>
#[argh(subcommand, name = "compile")]
pub(crate) struct CompileSubCommand {
    /// recursively compile files in a given folder
    #[argh(switch, short = 'r')]
    recursive: bool,

    // todo for html and pdf - make sure both are not checked at the same time
    /// specify we want our output as html file(s)
    #[argh(switch)]
    html: bool,

    /// specify we want our output as pdf file(s)
    #[argh(switch)]
    pdf: bool,

    /// specify output to a specific file - must be either pdf or html file extension
    // todo: parse output into valid file format - if can't, throw error
    #[argh(option, short = 'o')]
    output: Option<String>,

    /// write output files recursively into a folder
    #[argh(option, long="write-recursive", short = 'w')]
    write_recursive: Option<String>,

    // (maybe): parse input into file or folder
    /// input file/folder
    #[argh(positional)]
    input: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Usage: mdtex watch <flags> <input>
#[argh(subcommand, name = "watch")]
pub(crate) struct WatchSubCommand {
    // (maybe): parse input into file or folder
    /// input file/folder
    #[argh(positional)]
    input: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Usage: mdtex init <flags> <project-name>
#[argh(subcommand, name = "init")]
pub(crate) struct InitSubCommand {
    // parse input into folder?
    #[argh(positional)]
    project_name: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Usage: mdtex add <flags> <package-name>
#[argh(subcommand, name = "add")]
pub(crate) struct AddSubCommand {
    #[argh(positional)]
    add: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Usage: mdtex build <flags>
#[argh(subcommand, name = "build")]
pub(crate) struct BuildSubCommand {
    #[argh(positional)]
    build: String,
}