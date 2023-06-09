//! Stringify DocumentNode to html string.

use crate::model::html::*;

/// Stringify DocumentNode to html string.
///
/// This contains some options.
#[derive(Debug, Clone)]
pub struct Stringifier {
    /// Whether to format the output. Default is false.
    pub format: bool,
    /// The width of the line to break the code and indent. Default is 20.
    pub width: u32,
}

impl Default for Stringifier {
    fn default() -> Self {
        Self {
            format: false,
            width: 20,
        }
    }
}

impl Stringifier {
    /// Create a new Stringifier.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set whether to format the output.
    pub fn format(mut self, format: bool) -> Self {
        self.format = format;
        self
    }

    /// Set the width of the output.
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }
}

fn tag_to_str(tag: ElementTag) -> &'static str {
    match tag {
        ElementTag::Div => "div",
        ElementTag::Span => "span",
        ElementTag::P => "p",
        ElementTag::H1 => "h1",
        ElementTag::H2 => "h2",
        ElementTag::H3 => "h3",
        ElementTag::H4 => "h4",
        ElementTag::H5 => "h5",
        ElementTag::H6 => "h6",
        ElementTag::Ul => "ul",
        ElementTag::Ol => "ol",
        ElementTag::Li => "li",
        ElementTag::Blockquote => "blockquote",
        ElementTag::A => "a",
        ElementTag::Strong => "strong",
        ElementTag::Em => "em",
        ElementTag::Br => "br",
    }
}

impl Stringifier {
    /// Stringify DocumentNode to html string.
    pub fn stringify(&self, document: DocumentNode) -> String {
        let list = document
            .root
            .into_iter()
            .map(|node| self.stringify_node(node))
            .collect::<Vec<_>>();

        if self.format {
            list.join("\n")
        } else {
            list.join("")
        }
    }

    fn stringify_node(&self, node: Node) -> String {
        match node {
            Node::Element(element) => self.stringify_element(element),
            Node::Text(text) => self.stringify_text(text),
        }
    }

    fn stringify_element(&self, element: ElementNode) -> String {
        let tag = tag_to_str(element.tag);

        match element.tag {
            ElementTag::Br => format!("<{tag}>"),
            _ => {
                let mut attrs = String::new();

                if !element.class.is_empty() {
                    attrs += &format!(
                        " class=\"{}\"",
                        element.class.into_iter().collect::<Vec<_>>().join(" ")
                    );
                }

                if !element.id.is_empty() {
                    attrs += &format!(
                        " id=\"{}\"",
                        element.id.into_iter().collect::<Vec<_>>().join(" ")
                    );
                }

                if let Some(href) = element.href {
                    attrs += &format!(" href=\"{href}\"");
                }

                attrs += &element
                    .attrs
                    .iter()
                    .map(|(name, value)| format!(" {name}=\"{value}\""))
                    .collect::<String>();

                let list = element
                    .children
                    .iter()
                    .cloned()
                    .map(|node| self.stringify_node(node))
                    .collect::<Vec<_>>();

                let inner = if self.format {
                    if element.children.len() == 1 {
                        let child = list[0].clone();

                        if child.len() >= self.width as usize {
                            let child = Self::add_indent(&child);

                            format!("\n{child}\n")
                        } else {
                            child
                        }
                    } else if !element.children.iter().any(|node| node.is_block_item()) {
                        list.join("")
                    } else {
                        let children = list.join("\n");

                        let children = Self::add_indent(&children);

                        format!("\n{children}\n")
                    }
                } else {
                    list.join("")
                };

                format!("<{tag}{attrs}>{inner}</{tag}>")
            }
        }
    }

    fn stringify_text(&self, text: TextNode) -> String {
        text.text.to_string()
    }

    fn add_indent(input: &str) -> String {
        input
            .lines()
            .map(|line| String::from("    ") + line)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stringify() {
        let document = DocumentNode {
            root: vec![Node::Element(ElementNode {
                tag: ElementTag::P,
                children: vec![Node::Text(TextNode {
                    text: "Hello, world!".into(),
                })],
                ..Default::default()
            })],
        };

        let stringifier = Stringifier::new();

        assert_eq!(
            stringifier.stringify(document),
            "<p>Hello, world!</p>".to_string()
        );

        let document = DocumentNode {
            root: vec![Node::Element(ElementNode {
                tag: ElementTag::P,
                children: vec![
                    Node::Text(TextNode {
                        text: "Hello, ".into(),
                    }),
                    Node::Element(ElementNode {
                        tag: ElementTag::Strong,
                        children: vec![Node::Text(TextNode {
                            text: "world".into(),
                        })],
                        ..Default::default()
                    }),
                    Node::Text(TextNode { text: "!".into() }),
                    Node::Element(ElementNode {
                        tag: ElementTag::Br,
                        ..Default::default()
                    }),
                    Node::Text(TextNode {
                        text: "Hello, ".into(),
                    }),
                    Node::Element(ElementNode {
                        tag: ElementTag::Strong,
                        children: vec![Node::Text(TextNode {
                            text: "world".into(),
                        })],
                        ..Default::default()
                    }),
                    Node::Text(TextNode { text: "!".into() }),
                ],
                ..Default::default()
            })],
        };

        assert_eq!(
            stringifier.stringify(document),
            "<p>Hello, <strong>world</strong>!<br>Hello, <strong>world</strong>!</p>".to_string()
        );
    }

    #[test]
    fn test_stringify_attrs() {
        let document = DocumentNode {
            root: vec![Node::Element(ElementNode {
                tag: ElementTag::P,
                class: vec!["test".into(), "test2".into()],
                id: vec!["ttt".into()],
                href: Some("https://example.com".into()),
                attrs: vec![
                    ("data-test".into(), "ok".into()),
                    ("data-test2".into(), "ok2".into()),
                ],
                children: vec![Node::Text(TextNode {
                    text: "Hello, world!".into(),
                })],
                ..Default::default()
            })],
        };

        let stringifier = Stringifier::new();

        assert_eq!(
            stringifier.stringify(document),
            "<p class=\"test test2\" id=\"ttt\" href=\"https://example.com\" data-test=\"ok\" data-test2=\"ok2\">Hello, world!</p>".to_string()
        );
    }
}
