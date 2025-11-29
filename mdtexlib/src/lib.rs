mod ir;
mod parser;
mod renderer;

#[cfg(test)]
mod tests;

pub use parser::parse_to_raw_blocks;
pub use renderer::render;
