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
