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

pub fn compile(raw_string: impl Deref<Target = str>, path: impl AsRef<Path>) -> io::Result<()> {
    let raw_blocks = parse_to_raw_blocks(raw_string);
    let tokens = lower_raw_blocks(raw_blocks);
    let page = render(tokens);
    fs::write(path, page.to_html_string())
}
