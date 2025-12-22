use std::error::Error;

use build_html::{HtmlElement, HtmlTag};
use katex::OutputType;

pub fn render_math(math_block: &str, display_mode: bool) -> Result<String, Box<dyn Error>> {
    let opts = katex::Opts::builder()
        .display_mode(display_mode)
        .output_type(OutputType::Mathml)
        .build()
        .unwrap();

    let math = katex::render_with_opts(math_block, &opts)?;
    println!("{}", math);

    Ok(math)
}