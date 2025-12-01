use regex::Regex;
use std::ops::Deref;
use std::sync::LazyLock;

#[cfg(test)]
mod tests;
///
/// A raw version of a [BlockToken]. May contain invalid markdown, to be cleaned when lowered into
/// a [BlockToken]. For example, may containing a Heading 32
#[derive(Debug)]
pub enum RawBlock {
    Heading {
        raw_content: String,
        level: u32,
    },
    OrderedList(Vec<String>),
    UnorderedList(Vec<String>),
    BlockCode {
        content: String,
        language: Option<String>,
    },
    BlockMath(String),
    RawParagraph(String),
    RawTable(String),
}

impl RawBlock {
    /// Constructs a RawBlock from a string equivalent to a variant of RawBlock
    pub fn from_string(raw_string: impl Deref<Target = str>) -> Self {
        dbg!(raw_string.to_string());
        if let Some(head) = RawBlock::try_extract_header(&raw_string) {
            return head;
        } else if let Some(code) = RawBlock::try_extract_code(&raw_string) {
            return code;
        } else if let Some(math) = RawBlock::try_extract_math(&raw_string) {
            return math;
        } else if let Some(_) = RawBlock::try_extract_ol_item(&raw_string) {
            let mut output: Vec<String> = Vec::new();
            let mut valid = true;
            for l in raw_string.lines() {
                // TODO: rewrite this using some nicer method chaining
                if let Some(item) = RawBlock::try_extract_ol_item(&l) {
                    output.push(item);
                } else {
                    valid = false;
                    break;
                }
            }

            if valid {
                return RawBlock::OrderedList(output);
            }
        } else if let Some(_) = RawBlock::try_extract_ul_item(&raw_string) {
            let mut output: Vec<String> = Vec::new();
            let mut valid = true;
            for l in raw_string.lines() {
                // TODO: rewrite this using some nicer method chaining
                if let Some(item) = RawBlock::try_extract_ul_item(&l) {
                    output.push(item);
                } else {
                    valid = false;
                    break;
                }
            }

            if valid {
                return RawBlock::UnorderedList(output);
            }
        }

        RawBlock::RawParagraph(raw_string.to_owned())
    }

    // Either extracts a valid [RawBlock::Header] or returns `None`
    fn try_extract_header(raw_string: &impl Deref<Target = str>) -> Option<RawBlock> {
        static HEADER_REGEX: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"^(#{1,6})\s+(.+)").expect("Should be able to construct Regex")
        });

        HEADER_REGEX
            .captures(raw_string)
            .map(|caps| RawBlock::Heading {
                raw_content: caps[2].to_string(),
                level: caps[1].len() as u32,
            })
    }

    // Either extracts a valid [RawBlock::Math] or returns `None`
    fn try_extract_math(raw_string: &impl Deref<Target = str>) -> Option<RawBlock> {
        static CODE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"^\${2}\n(?s)(.+)(?-s)\${2}").expect("Should be able to construct Regex")
        });

        CODE_REGEX
            .captures(raw_string)
            .map(|caps| RawBlock::BlockMath(caps[1].to_string()))
    }

    // Either extracts a valid [RawBlock::Code] or returns `None`
    fn try_extract_code(raw_string: &impl Deref<Target = str>) -> Option<RawBlock> {
        static CODE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"^`{3}(.+)?\n(?s)(.+)(?-s)`{3}").expect("Should be able to construct Regex")
        });

        CODE_REGEX
            .captures(raw_string)
            .map(|caps| RawBlock::BlockCode {
                language: caps.get(1).map(|m| m.as_str().to_owned()),
                content: caps[2].to_string(),
            })
    }

    /// Either extracts a string corresponding to a valid ordered list item or returns None
    fn try_extract_ol_item(raw_string: &impl Deref<Target = str>) -> Option<String> {
        // TODO: add ability to leave items blank, current RegEx requires content after the list
        // item start.
        static OL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+\.\s+(.+)").unwrap());

        OL_REGEX.captures(raw_string).map(|caps| caps[1].into())
    }

    /// Either extracts a string corresponding to a valid unordered list item or returns None
    fn try_extract_ul_item(raw_string: &impl Deref<Target = str>) -> Option<String> {
        static UL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^-\s+(.+)?").unwrap());

        UL_REGEX.captures(raw_string).map(|caps| caps[1].into())
    }
}

enum ParseStateMachine {
    Nop,
    Paragraph,
    Header,
    MathBlock,
    CodeBlock,
    List,
}

/// This function
pub fn parse_to_raw_blocks(source: impl Deref<Target = str>) -> Vec<RawBlock> {
    todo!()
}
