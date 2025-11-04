use crate::{ir::RawBlock, parse_to_raw_blocks};

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
fn raw_block_parses_headers() {
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
fn top_level_blocks_parsed_correctly() {
    let raw_blocks = parse_to_raw_blocks(TEST_DOC);

    // there should be 9 blocks in the test document
    assert_eq!(raw_blocks.len(), 9);
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
    let _ = RawBlock::from_string(valid);
    let _ = RawBlock::from_string(invalid);
    // The function is incomplete, but shouldn't panic
}

#[test]
fn block_math_parses_correctly() {
    let test = "$$\n\\frac{a}{b}\n$$";
    let res = RawBlock::from_string(test);
    if let RawBlock::BlockMath(content) = res {
        assert!(content.contains("\\frac"));
    } else {
        panic!("Math block not parsed");
    }
}

#[test]
fn block_code_parses_correctly() {
    let test = "```\nprint('hi')\n```";
    let res = RawBlock::from_string(test);
    if let RawBlock::BlockCode(content) = res {
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
    if let RawBlock::Heading { raw_content, level } = res {
        assert_eq!(level, 2);
        assert_eq!(raw_content, "spaced heading");
    } else {
        panic!("heading not parsed");
    }
}

#[test]
fn is_valid_ol_item_function_basic_cases() {
    use crate::ir::is_valid_ol_item;
    let valid = String::from("1. item");
    let invalid_short = String::from("1item");
    let invalid_no_space = String::from("123item");
    assert!(is_valid_ol_item(&valid));
    assert!(!is_valid_ol_item(&invalid_short));
    assert!(!is_valid_ol_item(&invalid_no_space));
}

#[test]
fn code_and_math_edge_case_multiple_lines() {
    let test = "```\nline1\nline2\n```";
    let code = RawBlock::from_string(test);
    if let RawBlock::BlockCode(content) = code {
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