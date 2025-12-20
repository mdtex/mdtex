use mdtexcli::{cli::MDTeXTopLevelCLI, run};

fn main() {
    let args: MDTeXTopLevelCLI = argh::from_env();
    run(args);
}