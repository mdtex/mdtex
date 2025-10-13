use std::path::PathBuf;

pub enum MdtexIR {
    Markdown(MarkdownElements),
    LatexMath(String),
    LatexFigure(String, PathBuf),
}

pub enum MarkdownElements {
    Heading(String, u8),
    Para(String),
    InlineCode(String),
    Code(String),
}
