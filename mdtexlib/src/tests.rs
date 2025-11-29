use crate::ir::RawBlock;

const TEST_DOC: &str = r#"
# This is a header
## This is a header 2

This is standalone text.
This is standalone text in the same paragraph.

$$
\frac{6}{7}
$$

```
print("hello from a code block")
```

This paragraph has `code` and inline `\frac{6}{7}`

1. This is an ordered list
1. Item 2

- Unordered list
- Unordered list second item

- This is a separate unordered list
"#;

#[test]
fn raw_block_parses_headings() {
    let test_string = "# this is a heading one";

    let h1 = RawBlock::from_string(test_string);

    if let RawBlock::Heading { raw_content, level } = h1 {
        assert_eq!(level, 1);
        assert_eq!(raw_content, "this is a heading one");
    } else {
        panic!("did not correctly parse into Header");
    }

    let test_string2 = "# this is a heading one with `code` and $math$";

    let h2 = RawBlock::from_string(test_string2);

    if let RawBlock::Heading { raw_content, level } = h2 {
        assert_eq!(level, 1);
        assert_eq!(raw_content, "this is a heading one with `code` and $math$");
    } else {
        panic!("did not correctly parse into Header");
    }

    let test_string3 = "#### this is a heading one with `code` and $math$";

    let h3 = RawBlock::from_string(test_string3);

    if let RawBlock::Heading { raw_content, level } = h3 {
        assert_eq!(level, 4);
        assert_eq!(raw_content, "this is a heading one with `code` and $math$");
    } else {
        panic!("did not correctly parse into Header");
    }

    let test_string4 = "#this is not a header";

    let not_heading = RawBlock::from_string(test_string4);
    assert!(!matches!(
        not_heading,
        RawBlock::Heading {
            raw_content: _,
            level: _
        }
    ));
}

#[test]
fn unordered_list_parsed_correctly() {
    let test = "- item1\n- item2\n- item3";
    let res = RawBlock::from_string(test);
    if let RawBlock::UnorderedList(items) = res {
        assert_eq!(items, vec!["item1", "item2", "item3"]);
    } else {
        panic!("Unordered list not parsed");
    }
}

#[test]
fn malformed_unordered_list_falls_back_to_paragraph() {
    let test = "- item1\nnot a list\n- item3";
    let res = RawBlock::from_string(test);
    matches!(res, RawBlock::RawParagraph(_));
}

#[test]
fn ordered_list_valid_and_invalid_detection() {
    // This checks is_valid_ol_item behavior indirectly
    // and the RawBlock path for ordered lists (incomplete branch)
    let valid = "1. first\n2. second";
    let invalid = "a. not valid";
    let valid_ol = RawBlock::from_string(valid);
    let not_ol = RawBlock::from_string(invalid);

    if let RawBlock::OrderedList(contents) = valid_ol {
        dbg!(&contents);
        assert_eq!(contents, vec!["first", "second"]);
    } else {
        panic!("Should be able to get list items");
    }

    assert!(!matches!(not_ol, RawBlock::OrderedList(_)))
}

#[test]
fn block_math_parses_correctly() {
    let test = "$$\n\\frac{a}{b}\n$$";
    let res = RawBlock::from_string(test);
    if let RawBlock::BlockMath(content) = res {
        assert!(content == "\\frac{a}{b}\n");
    } else {
        panic!("Math block not parsed");
    }
}

#[test]
fn block_code_parses_correctly() {
    let test = "```\nprint('hi')\n```";
    let res = RawBlock::from_string(test);
    if let RawBlock::BlockCode {
        content,
        language: _,
    } = res
    {
        assert!(content.contains("print"));
    } else {
        panic!("Code block not parsed");
    }
}

#[test]
fn raw_paragraph_default_fallback() {
    let test = "This is just normal text.";
    let res = RawBlock::from_string(test);
    if let RawBlock::RawParagraph(t) = res {
        assert_eq!(t, test);
    } else {
        panic!("Expected RawParagraph fallback");
    }
}

#[test]
fn invalid_heading_structure_falls_back_to_paragraph() {
    let test = "###No space after hash";
    let res = RawBlock::from_string(test);
    matches!(res, RawBlock::RawParagraph(_));
}

#[test]
fn heading_with_extra_spaces() {
    let test = "##    spaced heading";
    let res = RawBlock::from_string(test);
    dbg!(&res);
    if let RawBlock::Heading { raw_content, level } = res {
        assert_eq!(level, 2);
        assert_eq!(raw_content, "spaced heading");
    } else {
        panic!("heading not parsed");
    }
}

#[test]
fn code_and_math_edge_case_multiple_lines() {
    let test = "```\nline1\nline2\n```";
    let code = RawBlock::from_string(test);
    if let RawBlock::BlockCode {
        content,
        language: _,
    } = code
    {
        assert!(content.contains("line1"));
    }

    let test2 = "$$\nmath1\nmath2\n$$";
    let math = RawBlock::from_string(test2);
    if let RawBlock::BlockMath(content) = math {
        assert!(content.contains("math1"));
    }
}

#[test]
fn paragraph_with_hyphen_not_list() {
    let test = "This - is not a list item";
    let res = RawBlock::from_string(test);
    assert!(matches!(res, RawBlock::RawParagraph(_)));
}

#[test]
fn heading_levels_and_spacing() {
    let cases = vec![
        ("# h1", 1, "h1"),
        ("## h2", 2, "h2"),
        ("###### h6", 6, "h6"),
        ("#     spaced", 1, "spaced"),
        ("###    nice", 3, "nice"),
    ];

    for (src, level, content) in cases {
        if let RawBlock::Heading {
            raw_content,
            level: lvl,
        } = RawBlock::from_string(src)
        {
            assert_eq!(lvl, level);
            assert_eq!(raw_content, content);
        } else {
            panic!("Failed heading parse for: {}", src);
        }
    }
}

#[test]
fn malformed_headings_fallback() {
    let cases = vec!["#no-space", "###no", "######### too many hashes"];

    for src in cases {
        assert!(matches!(
            RawBlock::from_string(src),
            RawBlock::RawParagraph(_)
        ));
    }
}

#[test]
fn unordered_list_rejects_inconsistent_items() {
    let src = "- a\nnot-a-list\n- b";
    assert!(matches!(
        RawBlock::from_string(src),
        RawBlock::RawParagraph(_)
    ));
}

#[test]
fn ordered_list_parses_multiple_items() {
    let src = "1. one\n2. two\n3. three";
    if let RawBlock::OrderedList(items) = RawBlock::from_string(src) {
        assert_eq!(items, vec!["one", "two", "three"]);
    } else {
        panic!("Ordered list not parsed properly");
    }
}

#[test]
fn block_code_preserves_intermediate_newlines() {
    let src = "```\nline1\nline2\nline3\n```";
    if let RawBlock::BlockCode {
        content,
        language: _,
    } = RawBlock::from_string(src)
    {
        assert!(content.contains("line1\n"));
        assert!(content.contains("line2\n"));
        assert!(content.contains("line3\n"));
    } else {
        panic!("BlockCode did not parse correctly");
    }
}

#[test]
fn block_math_preserves_newlines() {
    let src = "$$\na\nb\nc\n$$";
    if let RawBlock::BlockMath(math) = RawBlock::from_string(src) {
        assert!(math.contains("a\n"));
        assert!(math.contains("b\n"));
        assert!(math.contains("c\n"));
    } else {
        panic!("BlockMath newline preservation failed");
    }
}

#[test]
fn block_math_rejects_single_dollar_sign() {
    // why is this getting turned into a paragraph? Inline math is not a block level token, but a
    // lower level token
    let src = "$ a/b $";
    assert!(matches!(
        RawBlock::from_string(src),
        RawBlock::RawParagraph(_)
    ));
}

#[test]
fn paragraph_fallback_cases() {
    let cases = vec![
        "",
        "just a paragraph",
        "not a list - just hyphens",
        "1.not-a-list",
        "\n\nmulti\nline\nnot block\n",
    ];

    for src in cases {
        assert!(matches!(
            RawBlock::from_string(src),
            RawBlock::RawParagraph(_)
        ));
    }
}

#[test]
fn mixed_lists_not_confused() {
    let src = "1. one\n- two";
    // mixed list should not parse; should fall back
    assert!(matches!(
        RawBlock::from_string(src),
        RawBlock::RawParagraph(_)
    ));
}

#[test]
fn block_code_with_language_marker_still_parses() {
    // Many Markdown variants allow ```rust
    let src = "```rust\nfn main(){}\n```";
    if let RawBlock::BlockCode { content, language } = RawBlock::from_string(src) {
        assert!(content.contains("fn main"));
        assert!(language.is_some_and(|l| l == "rust"));
    } else {
        panic!("Failed to parse code block with language marker");
    }
}

#[test]
fn code_block_with_blank_lines() {
    let src = "```\nline1\n\nline3\n```";
    if let RawBlock::BlockCode {
        content,
        language: _,
    } = RawBlock::from_string(src)
    {
        assert!(content.contains("\n\n")); // ensure blank line preserved
    } else {
        panic!("Code block with blank lines not parsed");
    }
}
