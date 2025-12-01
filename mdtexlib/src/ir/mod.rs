#[cfg(test)]
mod tests;

/// A `BlockToken` represents the broadest unit that the MDTeX parser will break MDTeX into. Each
/// `BlockToken` variant will either contain a string representing content that is block level,
/// like a code block or a collection of smaller tokens representing content that can contain
/// further structure.
#[derive(Debug)]
pub enum BlockToken {
    /// Corresponds to a heading
    Heading {
        content: Vec<InlineToken>,
        level: u32,
    },
    /// Corresponds to an ordered list
    OrderedList(Vec<ListItem>),
    /// Corresponds to an unordered list
    UnorderedList(Vec<ListItem>),
    /// Corresponds to an list of tasks
    TaskList(Vec<ListItem>),
    /// Corresponds to a markdown quote
    BlockQuotes(String),
    /// Corresponds to a block of code
    BlockCode(String),
    /// Corresponds to a block of Latex Math
    BlockMath(String),
    /// Corresponds to a standalone block of text (a paragraph)
    Paragraph(Vec<InlineToken>),
    /// Corresponds to a markdown table
    Table(Vec<InlineToken>),
}

/// An [InlineToken] is a token that appears inside a [BlockToken].
/// It represents the smallest meaningful unit of inline MDTeX content.
#[derive(Debug)]
pub enum InlineToken {
    /// Plain text content.
    PlainText(String),
    /// Inline code span.
    InlineCode(String),
    /// Inline math expression.
    InlineMath(String),
    /// Hyperlink.
    Link { url: String, text: String },
    /// Emphasized text.
    Italics(String),
    /// Strong/emphasized text.
    Bold(String),
    /// Inline image with URL and caption.
    Image { url: String, caption: String },
}

/// A list item newtype used within a [BlockToken].
#[derive(Debug)]
pub struct ListItem(pub Vec<InlineToken>);
