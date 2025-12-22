use crate::{ir::{BlockToken, InlineToken, ListItem}, renderer::inline_to_html};
use build_html::*;

/// Helper to normalize mathml strings
fn normalize_mathml(s: &str) -> String {
    s.replace('\u{00A0}', " ")
}

#[test]
fn test_inline_to_html() {
    let inline_tokens = vec![
        InlineToken::PlainText("Plain text".to_string()),
        InlineToken::InlineCode("Code".to_string()),
        InlineToken::InlineMath("\\int_0^{\\infty}x^2\\ dx".to_string()),
        InlineToken::Link { url: "https://github.com/mdtex/mdtex".to_string(), text: "MDTeX Github".to_string() },
        InlineToken::Italics("Italicized".to_string()),
        InlineToken::Bold("Bolded".to_string()),
        InlineToken::Image { url: "https://tr06.tech/static/media/Portrait.dcc2c882aa96eb889768.JPG".to_string(), caption: "Hey I know that guy".to_string() },
    ];

    let render: Vec<String> = inline_to_html(inline_tokens).into_iter().map(|elem| elem.to_html_string()).collect();

    let expected = vec![
        "Plain text",
        "<code>Code</code>",
        r#"<span class="katex"><math xmlns="http://www.w3.org/1998/Math/MathML"><semantics><mrow><msubsup><mo>∫</mo><mn>0</mn><mi mathvariant="normal">∞</mi></msubsup><msup><mi>x</mi><mn>2</mn></msup><mtext> </mtext><mi>d</mi><mi>x</mi></mrow><annotation encoding="application/x-tex">\int_0^{\infty}x^2\ dx</annotation></semantics></math></span>"#,
        r#"<a href="https://github.com/mdtex/mdtex">MDTeX Github</a>"#,
        "<i>Italicized</i>",
        "<b>Bolded</b>",
        r#"<img src="https://tr06.tech/static/media/Portrait.dcc2c882aa96eb889768.JPG" alt="Hey I know that guy"/>"#,
    ];
    
    for i in 0..render.len() {
        assert_eq!(normalize_mathml(expected[i]), normalize_mathml(&render[i]));
    }
}

#[test]
fn test_render_heading() {
    let ir_list: Vec<BlockToken> = (0..=6).into_iter().map(|x| {
        BlockToken::Heading { content: vec![
            InlineToken::PlainText(format!("Heading {}", x)),
        ], level: x }
    }).collect();
    
    let render = crate::renderer::render(ir_list);

    let expected_result = r#"<!DOCTYPE html><html><head></head><body><h1>Heading 0</h1><h1>Heading 1</h1><h2>Heading 2</h2><h3>Heading 3</h3><h4>Heading 4</h4><h5>Heading 5</h5><h6>Heading 6</h6></body></html>"#;

    assert_eq!(expected_result, render.to_html_string());
}

#[test]
fn test_render_ordered_list() {
    let ir_list: Vec<BlockToken> = vec![BlockToken::OrderedList((1..=3).into_iter().map(|x| {
        ListItem(vec![InlineToken::PlainText(format!("Item {}", x))])
    }).collect())];
    
    let render = crate::renderer::render(ir_list);

    let expected_result = r#"<!DOCTYPE html><html><head></head><body><ol><li>Item 1</li><li>Item 2</li><li>Item 3</li></ol></body></html>"#;

    assert_eq!(expected_result, render.to_html_string());
}

#[test]
fn test_render_unordered_list() {
    let ir_list: Vec<BlockToken> = vec![BlockToken::UnorderedList((1..=3).into_iter().map(|x| {
        ListItem(vec![InlineToken::PlainText(format!("Item {}", x))])
    }).collect())];
    
    let render = crate::renderer::render(ir_list);

    let expected_result = r#"<!DOCTYPE html><html><head></head><body><ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul></body></html>"#;

    assert_eq!(expected_result, render.to_html_string());
}

#[test]
fn test_render_block_code() {
    let ir_list: Vec<BlockToken> = vec![BlockToken::BlockCode{lang: Some("rust".to_string()), content: "println!(\"Hello, world!\");".to_string()}];
    
    let render = crate::renderer::render(ir_list);

    let expected_result = r#"<!DOCTYPE html><html><head></head><body><pre>println!("Hello, world!");</pre></body></html>"#;

    assert_eq!(expected_result, render.to_html_string());
}

#[test]
fn test_render_block_math() {
    let ir_list: Vec<BlockToken> = vec![BlockToken::BlockMath(r#"\int_0^{\infty}x^2\ dx"#.to_string())];
    
    let render = crate::renderer::render(ir_list);

    let expected_result = r#"<!DOCTYPE html><html><head></head><body><span class="katex"><math xmlns="http://www.w3.org/1998/Math/MathML" display="block"><semantics><mrow><msubsup><mo>∫</mo><mn>0</mn><mi mathvariant="normal">∞</mi></msubsup><msup><mi>x</mi><mn>2</mn></msup><mtext> </mtext><mi>d</mi><mi>x</mi></mrow><annotation encoding="application/x-tex">\int_0^{\infty}x^2\ dx</annotation></semantics></math></span></body></html>"#;

    assert_eq!(normalize_mathml(expected_result), normalize_mathml(&render.to_html_string()));
}

#[test]
fn test_render_paragraph() {
    let ir_list: Vec<BlockToken> = vec![BlockToken::Paragraph(vec![InlineToken::PlainText("Paragraph".to_string())])];
    
    let render = crate::renderer::render(ir_list);

    let expected_result = r#"<!DOCTYPE html><html><head></head><body><p>Paragraph</p></body></html>"#;

    assert_eq!(normalize_mathml(expected_result), normalize_mathml(&render.to_html_string()));
}

#[test]
fn test_render_math_err() {
    let ir_list: Vec<BlockToken> = vec![
        BlockToken::BlockMath("\\this error? }".to_string())
    ];

    let render = crate::renderer::render(ir_list);
    
    let expected_result = r#"<!DOCTYPE html><html><head></head><body><div>\this error? }</div></body></html>"#;

    assert_eq!(expected_result, render.to_html_string());

    let inline_tokens = vec![
        InlineToken::InlineMath("\\this error? }".to_string()),
    ];

    let render = inline_to_html(inline_tokens)[0].to_html_string();
    
    let expected_result = r#"\this error? }"#;

    assert_eq!(expected_result, render.to_html_string());
}