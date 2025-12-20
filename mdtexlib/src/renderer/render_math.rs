use std::error::Error;

use build_html::{HtmlElement, HtmlTag};
use katex::OutputType;

pub fn render_math(math_block: &str) -> Result<String, Box<dyn Error>> {
    let opts = katex::Opts::builder()
        .display_mode(true)
        .output_type(OutputType::Mathml)
        .build()
        .unwrap();

    let math = katex::render_with_opts(math_block, &opts)?;

    Ok(math)
}