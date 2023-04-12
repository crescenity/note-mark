use crate::model::html::*;

#[derive(Debug, Clone)]
pub struct Stringifier {
    format: bool,
    width: u32,
}

impl Default for Stringifier {
    fn default() -> Self {
        Self {
            format: false,
            width: 100,
        }
    }
}

impl Stringifier {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn format(mut self, format: bool) -> Self {
        self.format = format;
        self
    }

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
    pub fn stringify(&self, document: DocumentNode) -> String {
        if self.format {
            document
                .root
                .into_iter()
                .map(|node| self.stringify_node(node))
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            document
                .root
                .into_iter()
                .map(|node| self.stringify_node(node))
                .collect::<Vec<_>>()
                .join("")
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
                if self.format {
                    if element.children.len() == 1 {
                        let children = self.stringify_node(element.children[0].clone());

                        if children.len() >= self.width as usize {
                            let children = children
                                .lines()
                                .map(|line| String::from("    ") + line)
                                .collect::<Vec<_>>()
                                .join("\n");
                            format!("\n<{tag}>\n{children}\n</{tag}>\n")
                        } else {
                            format!("<{tag}>{children}</{tag}>")
                        }
                    } else if element
                        .children
                        .iter()
                        .filter(|node| node.is_block_item())
                        .count()
                        == 0
                    {
                        let children = element
                            .children
                            .into_iter()
                            .map(|node| self.stringify_node(node))
                            .collect::<Vec<_>>()
                            .join("");

                        format!("<{tag}>{children}</{tag}>")
                    } else {
                        let children = element
                            .children
                            .into_iter()
                            .map(|node| self.stringify_node(node))
                            .collect::<Vec<_>>()
                            .join("\n")
                            .lines()
                            .map(|line| String::from("    ") + line)
                            .collect::<Vec<_>>()
                            .join("\n");

                        format!("<{tag}>\n{children}\n</{tag}>")
                    }
                } else {
                    let children = element
                        .children
                        .into_iter()
                        .map(|node| self.stringify_node(node))
                        .collect::<Vec<_>>()
                        .join("");

                    format!("<{tag}>{children}</{tag}>")
                }
            }
        }
    }

    fn stringify_text(&self, text: TextNode) -> String {
        text.text.to_string()
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
}
