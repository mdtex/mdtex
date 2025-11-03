use std::ops::Deref;

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
    Paragraph(Vec<InlineToken>), // corresponds to the standalone text type Agastya and I discussed - MFR
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

/// A raw version of a [BlockToken]. May contain invalid markdown, to be cleaned when lowered into
/// a [BlockToken]. For example, may containing a Heading 32
pub enum RawBlock {
    Heading { raw_content: String, level: u32 },
    OrderedList(Vec<String>),
    UnorderedList(Vec<String>),
    BlockCode(String),
    BlockMath(String),
    RawParagraph(String),
    RawTable(String),
}

impl RawBlock {
    /// Constructs a RawBlock from a string equivalent to a variant of RawBlock
    pub fn from_string(raw_string: impl Deref<Target = str>) -> Self {
        // One important question is, whose job is it to make sure that this function gets perfect
        // inputs? Should this function itself reject imperfect inputs? Should it return an Option
        // or Result based on whether an input is usable? Should the parser itself manage what
        // inputs this function gets. This implementation assumes the latter.
        if raw_string.starts_with('#') && raw_string.find('\n').is_none() {
            // this *may* be a heading, so we can go ahead and try to construct one.
            // we can try to see if this is truly a header by getting the index of the first
            // space, then checking to see if the string starts with that many #s
            // In theory, this should work because all headings look like this:
            // "###### blah blah blah"
            if let Some(idx) = raw_string.find(' ') {
                let possible_heading = "#".repeat(idx);
                if raw_string.starts_with(&possible_heading) {
                    let raw_content = raw_string.strip_prefix(&possible_heading).unwrap();
                    return RawBlock::Heading {
                        raw_content: raw_content.trim().into(),
                        level: idx as u32,
                    };
                }
            }
        } else if raw_string.starts_with("- ") {
            // unordered lists are fairly easy, we split on lines and insert them into the variant
            // We are going to assume for now that no bad inputs will be fed to this associated
            // function
            let mut lines = raw_string.lines();

            if lines.all(|x| x.starts_with("- ")) {
                return RawBlock::UnorderedList(
                    lines
                        .map(|l| l.strip_prefix('-').unwrap().trim().to_string())
                        .collect(),
                );
            }
        } else if is_valid_ol_item(&raw_string) {
            // Ordered lists will be slightly harder, as technically, any numeric characters
            // before a . at the start of a line is a valid markdown ordered list item
            let mut lines = raw_string.lines();

            if lines.all(|x| is_valid_ol_item(&x)) {
                return RawBlock::OrderedList(
                    lines.map(|l| l.spli.collect());
            }
        } else if raw_string.starts_with("$$") {
            // Again, we will assume that this is a perfectly valid LaTeX Math block, that starts
            // and ends with $$
            for l in raw_string.lines() {
                let mut math = String::new();
                if l != "$$" {
                    math.push_str(l);
                }

                return RawBlock::BlockMath(math);
            }
        } else if raw_string.starts_with("```") {
            // Again, we will assume that this is a perfectly valid LaTeX Math block, that starts
            // and ends with $$
            for l in raw_string.lines() {
                let mut code = String::new();
                if l != "```" {
                    code.push_str(l);
                }

                return RawBlock::BlockCode(code);
            }
        }

        // To owned because of type inference not working with .into(). Could also use to_string,
        // but might as well use Rust's type inference when we can
        RawBlock::RawParagraph(raw_string.to_owned())
    }
}

fn is_valid_ol_item(line: &impl Deref<Target = str>) -> bool {
    if let Some(idx) = line.find(' ') {
        if idx < 2 {return false}

        // use iterator to do logic, less expensive

        // for c in {
        //     if !c.is_ascii_digit() {
        //         return false;
        //     }
        // }

        true
    } else {
        false
    }
}

/// A list item newtype used within a [BlockToken].
#[derive(Debug)]
pub struct ListItem(Vec<InlineToken>);
