use build_html::escape_html;
use build_html::{HtmlChild, HtmlContainer, HtmlElement, HtmlPage, HtmlTag};

use crate::ir::{BlockToken, InlineToken, ListItem};

#[cfg(test)]
mod tests;

pub fn render(ir_list: Vec<BlockToken>) -> HtmlPage {
    let mut out = HtmlPage::new();
    for token in ir_list {
        match token {
            BlockToken::Heading { content, level } => {
                out.add_html(make_heading(level, content));
            }
            BlockToken::OrderedList(list_items) => out.add_html(make_ordered_list(list_items)),
            BlockToken::UnorderedList(list_items) => out.add_html(make_unordered_list(list_items)),
            BlockToken::BlockCode { lang: _, content } => {
                out.add_html(HtmlElement::new(HtmlTag::PreformattedText).with_child(content.into()))
            }
            BlockToken::BlockMath(m) => {
                out.add_html(HtmlElement::new(HtmlTag::Div).with_child(m.into()))
            }
            BlockToken::Paragraph(inline_tokens) => {
                out.add_html(make_paragraph(inline_tokens));
            }
        }
    }

    out
}

fn make_paragraph(content: Vec<InlineToken>) -> HtmlElement {
    let mut para = HtmlElement::new(HtmlTag::ParagraphText);

    let children = inline_to_html(content);

    for child in children {
        para.add_child(child);
    }

    para
}

// ugly ass function
fn make_unordered_list(items: Vec<ListItem>) -> HtmlElement {
    let mut list = HtmlElement::new(HtmlTag::UnorderedList);

    for i in items {
        let mut li = HtmlElement::new(HtmlTag::ListElement);
        let children = inline_to_html(i.0);

        for c in children {
            li.add_child(c);
        }

        list.add_child(li.into());
    }

    list
}

fn make_ordered_list(items: Vec<ListItem>) -> HtmlElement {
    let mut list = HtmlElement::new(HtmlTag::OrderedList);

    for i in items {
        let mut li = HtmlElement::new(HtmlTag::ListElement);
        let children = inline_to_html(i.0);

        for c in children {
            li.add_child(c);
        }

        list.add_child(li.into());
    }

    list
}

fn make_heading(level: u8, content: Vec<InlineToken>) -> HtmlElement {
    let mut heading = match level {
        0 => HtmlElement::new(HtmlTag::Heading1),
        1 => HtmlElement::new(HtmlTag::Heading1),
        2 => HtmlElement::new(HtmlTag::Heading2),
        3 => HtmlElement::new(HtmlTag::Heading3),
        4 => HtmlElement::new(HtmlTag::Heading4),
        5 => HtmlElement::new(HtmlTag::Heading5),
        _ => HtmlElement::new(HtmlTag::Heading6),
    };

    for elem in inline_to_html(content) {
        heading.add_child(elem);
    }

    heading
}

fn inline_to_html(tokens: Vec<InlineToken>) -> Vec<HtmlChild> {
    let mut out = vec![];

    for token in tokens {
        match token {
            InlineToken::PlainText(t) => out.push(HtmlChild::Raw(escape_html(&t))),
            InlineToken::InlineCode(c) => out.push(HtmlChild::Element(
                HtmlElement::new(HtmlTag::CodeText).with_child(escape_html(&c).into()),
            )),
            InlineToken::InlineMath(m) => out.push(
                HtmlElement::new(HtmlTag::Div)
                    .with_child(escape_html(&m).into())
                    .into(),
            ),
            InlineToken::Link { url, text } => out.push(
                HtmlElement::new(HtmlTag::Link)
                    .with_child(escape_html(&text).into())
                    .with_attribute("href", url)
                    .into(),
            ),
            InlineToken::Italics(t) => out.push(
                HtmlElement::new(HtmlTag::Italic)
                    .with_child(escape_html(&t).into())
                    .into(),
            ),
            InlineToken::Bold(t) => out.push(
                HtmlElement::new(HtmlTag::Bold)
                    .with_child(escape_html(&t).into())
                    .into(),
            ),
            InlineToken::Image { url, caption } => out.push(
                HtmlElement::new(HtmlTag::Image)
                    .with_attribute("src", url)
                    .with_attribute("alt", caption)
                    .into(),
            ),
        }
    }

    out
}
