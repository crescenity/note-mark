use std::collections::HashSet;

use crate::model::html::*;

pub use config::*;

#[derive(Debug, Clone)]
pub struct TocMaker {
    level: u8,
    list_type: ListType,
}

pub mod config {
    use crate::html::ElementTag;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ListType {
        Unordered,
        Ordered,
    }

    impl ListType {
        pub fn to_tag(&self) -> ElementTag {
            match self {
                Self::Unordered => ElementTag::Ul,
                Self::Ordered => ElementTag::Ol,
            }
        }
    }
}

impl Default for TocMaker {
    fn default() -> Self {
        Self {
            level: 3,
            list_type: ListType::Unordered,
        }
    }
}

impl TocMaker {
    pub fn level(mut self, level: u8) -> Self {
        self.level = level;
        self
    }
}

impl TocMaker {
    pub fn make_toc<'a>(&self, input: &mut DocumentNode<'a>) -> DocumentNode<'a> {
        let mut list = vec![];

        let mut set = HashSet::new();

        for node in input.root.iter_mut() {
            let Node::Element(element) = node else { continue; };

            use ElementTag::*;

            let headline_level = match element.tag {
                H1 | H2 | H3 | H4 | H5 | H6 => element.tag.get_headline_level().unwrap(),
                _ => continue,
            };

            if headline_level > self.level {
                continue;
            }

            let text = get_text(&element.children);

            let (text, id) = if set.insert(text.clone()) {
                (text.clone(), text)
            } else {
                let mut index = 1;

                while !set.insert(text.clone() + &index.to_string()) {
                    index += 1;
                }

                (text.clone(), text + &index.to_string())
            };

            element.id.push(id.clone());

            list.push((headline_level, text, id));
        }

        let output = self.nest(&list);

        DocumentNode { root: vec![output] }
    }

    fn nest(&self, rest: &[(u8, String, String)]) -> Node<'static> {
        let mut rest = rest;

        let mut children = vec![];

        while !rest.is_empty() {
            let next = rest[1..]
                .iter()
                .position(|(level, _, _)| *level <= rest[0].0);

            let a_tag = Node::Element(ElementNode {
                tag: ElementTag::A,
                href: Some(String::from("#") + &rest[0].2),
                children: vec![Node::Text(TextNode {
                    text: rest[0].1.clone().into(),
                })],
                ..Default::default()
            });

            let mut element = ElementNode {
                tag: ElementTag::Li,
                children: vec![a_tag],
                ..Default::default()
            };

            if let Some(index) = next {
                // `index` is the index of the next element with the same or higher level
                let index = index + 1;

                if index != 1 {
                    let output = self.nest(&rest[1..index]);
                    element.children.push(output);
                }

                children.push(Node::Element(element));

                rest = &rest[index..];
            } else {
                if rest.len() >= 2 {
                    element.children.push(self.nest(&rest[1..]));
                }

                children.push(Node::Element(element));

                rest = &[];
            }
        }

        Node::Element(ElementNode {
            tag: self.list_type.to_tag(),
            children,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Markdown;

    #[test]
    fn test_make_toc() {
        let input =
            "# H1AAAAAA\n\n# H1AAAAAA\n\n# H1BBBBBB\n\n## H2AAAAAA\n\n## H2BBBBBB\n\n# H1CCCCCC\n\n";

        let markdown = Markdown::default();

        let tokens = Markdown::lex(input);
        let tree = markdown.parser.parse(input, tokens);
        let mut document = markdown.transformer.transform(tree);

        let toc = TocMaker::default().make_toc(&mut document);

        let output1 = markdown.stringifier.stringify(document);

        assert_eq!(output1, "<h1 id=\"H1AAAAAA\">H1AAAAAA</h1><h1 id=\"H1AAAAAA1\">H1AAAAAA</h1><h1 id=\"H1BBBBBB\">H1BBBBBB</h1><h2 id=\"H2AAAAAA\">H2AAAAAA</h2><h2 id=\"H2BBBBBB\">H2BBBBBB</h2><h1 id=\"H1CCCCCC\">H1CCCCCC</h1>");

        let output2 = markdown.stringifier.stringify(toc);

        assert_eq!(output2, "<ul><li><a href=\"#H1AAAAAA\">H1AAAAAA</a></li><li><a href=\"#H1AAAAAA1\">H1AAAAAA</a></li><li><a href=\"#H1BBBBBB\">H1BBBBBB</a><ul><li><a href=\"#H2AAAAAA\">H2AAAAAA</a></li><li><a href=\"#H2BBBBBB\">H2BBBBBB</a></li></ul></li><li><a href=\"#H1CCCCCC\">H1CCCCCC</a></li></ul>")
    }
}
