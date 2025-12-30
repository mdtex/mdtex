mod ir;
mod parser;
mod renderer;

#[cfg(test)]
mod tests;

use std::{fs, io, ops::Deref, path::Path};

use build_html::Html;
use parser::parse_to_raw_blocks;
use renderer::render;

use crate::ir::lower_raw_blocks;

pub fn compile(input_path: impl AsRef<Path>, output_path: impl AsRef<Path>) -> io::Result<()> {
    let raw_string = fs::read_to_string(input_path)?;
    let raw_blocks = parse_to_raw_blocks(raw_string);
    let tokens = lower_raw_blocks(raw_blocks);
    let page = render(tokens);
    fs::write(output_path, page.to_html_string())
}
