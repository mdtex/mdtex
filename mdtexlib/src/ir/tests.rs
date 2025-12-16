use super::*;
use crate::parser::RawBlock;

// ---------- Inline lowering tests ----------

#[test]
fn plain_text_only() {
    let tokens = super::lower_string_to_inline("hello world".to_string());
    assert_eq!(tokens.len(), 1);

    match &tokens[0] {
        InlineToken::PlainText(s) => assert_eq!(s, "hello world"),
        _ => panic!("expected PlainText"),
    }
}

#[test]
fn bold_and_plain_text() {
    let tokens = super::lower_string_to_inline("hi **there**!".to_string());

    assert_eq!(tokens.len(), 3);

    matches!(&tokens[0], InlineToken::PlainText(s) if s == "hi ");
    matches!(&tokens[1], InlineToken::Bold(s) if s == "there");
    matches!(&tokens[2], InlineToken::PlainText(s) if s == "!");
}

#[test]
fn italics_code_math() {
    let tokens = super::lower_string_to_inline("*a* `b` $c$".to_string());

    assert_eq!(tokens.len(), 5);

    assert!(matches!(tokens[0], InlineToken::Italics(ref s) if s == "a"));
    assert!(matches!(tokens[1], InlineToken::PlainText(ref s) if s == " "));
    assert!(matches!(tokens[2], InlineToken::InlineCode(ref s) if s == "b"));
    assert!(matches!(tokens[3], InlineToken::PlainText(ref s) if s == " "));
    assert!(matches!(tokens[4], InlineToken::InlineMath(ref s) if s == "c"));
}

#[test]
fn link_parsing() {
    let tokens = super::lower_string_to_inline("[text](url)".to_string());

    assert_eq!(tokens.len(), 1);

    match &tokens[0] {
        InlineToken::Link { text, url } => {
            assert_eq!(text, "text");
            assert_eq!(url, "url");
        }
        _ => panic!("expected Link"),
    }
}

#[test]
fn image_parsing() {
    let tokens = super::lower_string_to_inline("![cap](img.png)".to_string());

    assert_eq!(tokens.len(), 1);

    match &tokens[0] {
        InlineToken::Image { caption, url } => {
            assert_eq!(caption, "cap");
            assert_eq!(url, "img.png");
        }
        _ => panic!("expected Image"),
    }
}

#[test]
fn mixed_inline_content() {
    let tokens = super::lower_string_to_inline("Hello *world* with **rust**".to_string());

    dbg!(&tokens);
    assert_eq!(tokens.len(), 4);

    assert!(matches!(tokens[0], InlineToken::PlainText(ref s) if s == "Hello "));
    assert!(matches!(tokens[1], InlineToken::Italics(ref s) if s == "world"));
    assert!(matches!(tokens[2], InlineToken::PlainText(ref s) if s == " with "));
    assert!(matches!(tokens[3], InlineToken::Bold(ref s) if s == "rust"));
}

// ---------- Block lowering tests ----------

#[test]
fn heading_lowering() {
    let raw = vec![RawBlock::Heading {
        raw_content: "Hello **world**".to_string(),
        level: 2,
    }];

    let blocks = lower_raw_blocks(raw);
    assert_eq!(blocks.len(), 1);

    match &blocks[0] {
        BlockToken::Heading { content, level } => {
            assert_eq!(*level, 2);
            assert!(matches!(content[1], InlineToken::Bold(_)));
        }
        _ => panic!("expected Heading"),
    }
}

#[test]
fn paragraph_lowering() {
    let raw = vec![RawBlock::RawParagraph("text".to_string())];

    let blocks = lower_raw_blocks(raw);

    match &blocks[0] {
        BlockToken::Paragraph(tokens) => {
            assert_eq!(tokens.len(), 1);
            assert!(matches!(tokens[0], InlineToken::PlainText(_)));
        }
        _ => panic!("expected Paragraph"),
    }
}

#[test]
fn ordered_list_lowering() {
    let raw = vec![RawBlock::OrderedList(vec![
        "one".to_string(),
        "**two**".to_string(),
    ])];

    let blocks = lower_raw_blocks(raw);

    match &blocks[0] {
        BlockToken::OrderedList(items) => {
            assert_eq!(items.len(), 2);
            assert!(matches!(items[1].0[0], InlineToken::Bold(_)));
        }
        _ => panic!("expected OrderedList"),
    }
}

#[test]
fn block_code_and_math() {
    let raw = vec![
        RawBlock::BlockCode {
            content: "fn main() {}".to_string(),
            language: Some("rust".to_string()),
        },
        RawBlock::BlockMath("x^2".to_string()),
    ];

    let blocks = lower_raw_blocks(raw);

    assert!(matches!(
        blocks[0],
        BlockToken::BlockCode { lang: Some(_), .. }
    ));

    assert!(matches!(
        blocks[1],
        BlockToken::BlockMath(ref s) if s == "x^2"
    ));
}

#[test]
fn lowers_heading_block() {
    let raw = vec![RawBlock::Heading {
        raw_content: "Heading text".to_string(),
        level: 3,
    }];

    let lowered = lower_raw_blocks(raw);

    assert_eq!(lowered.len(), 1);

    match &lowered[0] {
        BlockToken::Heading { content, level } => {
            assert_eq!(*level, 3);
            assert!(!content.is_empty(), "heading content should not be empty");
        }
        _ => panic!("Expected Heading block"),
    }
}

#[test]
fn lowers_ordered_list_block() {
    let raw = vec![RawBlock::OrderedList(vec![
        "First".to_string(),
        "Second".to_string(),
    ])];

    let lowered = lower_raw_blocks(raw);

    match &lowered[0] {
        BlockToken::OrderedList(items) => {
            assert_eq!(items.len(), 2);
            for item in items {
                assert!(
                    !item.0.is_empty(),
                    "list items should contain inline tokens"
                );
            }
        }
        _ => panic!("Expected OrderedList"),
    }
}

#[test]
fn lowers_unordered_list_block() {
    let raw = vec![RawBlock::UnorderedList(vec![
        "Apple".to_string(),
        "Banana".to_string(),
    ])];

    let lowered = lower_raw_blocks(raw);

    match &lowered[0] {
        BlockToken::UnorderedList(items) => {
            assert_eq!(items.len(), 2);
        }
        _ => panic!("Expected UnorderedList"),
    }
}

#[test]
fn lowers_block_code() {
    let raw = vec![RawBlock::BlockCode {
        content: "fn main() {}".to_string(),
        language: Some("rust".to_string()),
    }];

    let lowered = lower_raw_blocks(raw);

    match &lowered[0] {
        BlockToken::BlockCode { lang, content } => {
            assert_eq!(lang.as_deref(), Some("rust"));
            assert_eq!(content, "fn main() {}");
        }
        _ => panic!("Expected BlockCode"),
    }
}

#[test]
fn lowers_block_math() {
    let raw = vec![RawBlock::BlockMath("x^2 + y^2".to_string())];

    let lowered = lower_raw_blocks(raw);

    match &lowered[0] {
        BlockToken::BlockMath(math) => {
            assert_eq!(math, "x^2 + y^2");
        }
        _ => panic!("Expected BlockMath"),
    }
}

#[test]
fn lowers_paragraph_block() {
    let raw = vec![RawBlock::RawParagraph("Just a paragraph".to_string())];

    let lowered = lower_raw_blocks(raw);

    match &lowered[0] {
        BlockToken::Paragraph(tokens) => {
            assert!(!tokens.is_empty(), "paragraph should contain inline tokens");
        }
        _ => panic!("Expected Paragraph"),
    }
}

#[test]
fn preserves_block_order() {
    let raw = vec![
        RawBlock::Heading {
            raw_content: "Title".to_string(),
            level: 1,
        },
        RawBlock::RawParagraph("Body, *italics*, **bold**, `lady` $hear me tonight$".to_string()),
        RawBlock::BlockMath("x = y".to_string()),
    ];

    let lowered = lower_raw_blocks(raw);

    assert_eq!(lowered.len(), 3);
    assert!(matches!(lowered[0], BlockToken::Heading { .. }));
    assert!(matches!(lowered[1], BlockToken::Paragraph(_)));
    assert!(matches!(lowered[2], BlockToken::BlockMath(_)));
}
