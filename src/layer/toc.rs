use crate::model::html::*;

pub struct MakeToc {
    level: u8,
}

impl Default for MakeToc {
    fn default() -> Self {
        Self { level: 3 }
    }
}

impl MakeToc {
    pub fn level(mut self, level: u8) -> Self {
        self.level = level;
        self
    }
}

impl MakeToc {
    pub fn make_toc<'a>(&self, input: &mut DocumentNode<'a>) -> DocumentNode<'a> {
        let mut root = vec![];

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

            fn push(list: &mut Vec<Node>, level: u8, label: String) {
                if level > 1 {
                    if let Some(Node::Element(last)) = list.last_mut() {
                        push(&mut last.children, level - 1, label);
                    }
                } else {
                    list.push(Node::Element(ElementNode {
                        tag: ElementTag::Ol,
                        id: vec![label],
                        class: vec![],
                        children: vec![],
                    }));
                }
            }

            push(&mut root, headline_level, get_text(&element.children));
        }

        DocumentNode { root }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Markdown;

    #[test]
    fn test_make_toc() {
        let input = "# H1A\n# H1B\n## H2A\n## H2B\n# H1C\n";

        let markdown = Markdown::default();

        let tokens = Markdown::lex(input);
        let tree = markdown.parser.parse(input, tokens);
        let mut document = markdown.transformer.transform(tree);

        let toc = MakeToc::default().make_toc(&mut document);

        let output1 = markdown.stringifier.stringify(document);

        println!("{output1}");

        let output2 = markdown.stringifier.stringify(toc);

        println!("{output2}");
    }
}
