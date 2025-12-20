use crate::parser::RawBlock;

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
        level: u8,
    },
    /// Corresponds to an ordered list
    OrderedList(Vec<ListItem>),
    /// Corresponds to an unordered list
    UnorderedList(Vec<ListItem>),
    /// Corresponds to a block of code
    BlockCode {
        lang: Option<String>,
        content: String,
    },
    /// Corresponds to a block of Latex Math
    BlockMath(String),
    /// Corresponds to a standalone block of text (a paragraph)
    Paragraph(Vec<InlineToken>),
}

/// An [InlineToken] is a token that appears inside a [BlockToken].
/// It represents the smallest meaningful unit of inline MDTeX content.
#[derive(Debug, PartialEq)]
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

pub fn lower_raw_blocks(raw_blocks: Vec<RawBlock>) -> Vec<BlockToken> {
    let mut out = vec![];
    for block in raw_blocks {
        match block {
            RawBlock::Heading { raw_content, level } => {
                let inline_tokens = lower_string_to_inline(raw_content);
                out.push(BlockToken::Heading {
                    content: inline_tokens,
                    level: level.try_into().unwrap(),
                });
            }
            RawBlock::OrderedList(items) => {
                out.push(BlockToken::OrderedList(
                    items
                        .iter()
                        .map(|item| lower_string_to_inline(item.to_string()))
                        .map(|tokens| ListItem(tokens))
                        .collect(),
                ));
            }
            RawBlock::UnorderedList(items) => {
                out.push(BlockToken::UnorderedList(
                    items
                        .iter()
                        .map(|item| lower_string_to_inline(item.to_string()))
                        .map(|tokens| ListItem(tokens))
                        .collect(),
                ));
            }
            RawBlock::BlockCode { content, language } => out.push(BlockToken::BlockCode {
                lang: language,
                content,
            }),
            RawBlock::BlockMath(math) => out.push(BlockToken::BlockMath(math)),
            RawBlock::RawParagraph(content) => {
                out.push(BlockToken::Paragraph(lower_string_to_inline(content)))
            }
        }
    }

    out
}

fn lower_string_to_inline(input: String) -> Vec<InlineToken> {
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < input.len() {
        let rest = &input[i..];

        // ---------- Image ----------
        if let Some(token) = try_image(rest) {
            i += token_len(&token);
            tokens.push(token);
            continue;
        }

        // ---------- Link ----------
        if let Some(token) = try_link(rest) {
            i += token_len(&token);
            tokens.push(token);
            continue;
        }

        // ---------- Bold ----------
        if let Some(token) = try_wrapped(rest, "**", |s| InlineToken::Bold(s)) {
            i += token_len(&token);
            tokens.push(token);
            continue;
        }

        // ---------- Italics ----------
        if let Some(token) = try_wrapped(rest, "*", |s| InlineToken::Italics(s)) {
            i += token_len(&token);
            tokens.push(token);
            continue;
        }

        // ---------- Inline Code ----------
        if let Some(token) = try_wrapped(rest, "`", |s| InlineToken::InlineCode(s)) {
            i += token_len(&token);
            tokens.push(token);
            continue;
        }

        // ---------- Inline Math ----------
        if let Some(token) = try_wrapped(rest, "$", |s| InlineToken::InlineMath(s)) {
            i += token_len(&token);
            tokens.push(token);
            continue;
        }

        // ---------- Plain text ----------
        let start = i;
        i += 1;
        while i < input.len() && !is_special(&input[i..]) {
            i += 1;
        }
        tokens.push(InlineToken::PlainText(input[start..i].to_string()));
    }

    tokens
}

fn try_wrapped<F>(input: &str, delim: &str, f: F) -> Option<InlineToken>
where
    F: Fn(String) -> InlineToken,
{
    if !input.starts_with(delim) {
        return None;
    }

    let rest = &input[delim.len()..];
    let end = rest.find(delim)?;
    let content = &rest[..end];

    Some(f(content.to_string()))
}

fn try_link(input: &str) -> Option<InlineToken> {
    if !input.starts_with('[') {
        return None;
    }

    let close = input.find("](")?;
    let text = &input[1..close];

    let rest = &input[close + 2..];
    let end = rest.find(')')?;
    let url = &rest[..end];

    Some(InlineToken::Link {
        text: text.to_string(),
        url: url.to_string(),
    })
}

fn try_image(input: &str) -> Option<InlineToken> {
    if !input.starts_with("![") {
        return None;
    }

    let close = input.find("](")?;
    let caption = &input[2..close];

    let rest = &input[close + 2..];
    let end = rest.find(')')?;
    let url = &rest[..end];

    Some(InlineToken::Image {
        caption: caption.to_string(),
        url: url.to_string(),
    })
}

fn token_len(token: &InlineToken) -> usize {
    match token {
        InlineToken::Bold(s) => s.len() + 4,
        InlineToken::Italics(s) => s.len() + 2,
        InlineToken::InlineCode(s) => s.len() + 2,
        InlineToken::InlineMath(s) => s.len() + 2,
        InlineToken::Link { text, url } => text.len() + url.len() + 4,
        InlineToken::Image { caption, url } => caption.len() + url.len() + 5,
        InlineToken::PlainText(s) => s.len(),
    }
}

fn is_special(s: &str) -> bool {
    s.starts_with('*')
        || s.starts_with('`')
        || s.starts_with('$')
        || s.starts_with('[')
        || s.starts_with("![")
}
