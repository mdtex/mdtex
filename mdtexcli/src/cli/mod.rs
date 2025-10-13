pub(crate) mod commands;

use commands::*;

use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Top-level command
pub struct MDTeXTopLevelCLI {
    #[argh(subcommand)]
    nested: MDTeXCommands,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
// Define our MDTeX commands:
/// Compile - compile given .mdt file(s)
/// Watch - watch a given file and recompile on save
/// Init - initialize a new MDTeX project
/// Add - add a given LaTeX package to the current MDTeX project
/// Build - build a given MDTeX project
enum MDTeXCommands {
    Compile(CompileSubCommand),
    Watch(WatchSubCommand),
    Init(InitSubCommand),
    Add(AddSubCommand),
    Build(BuildSubCommand),
}