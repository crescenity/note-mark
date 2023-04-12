mod layer;
mod model;

use layer::*;

pub use model::*;

#[derive(Debug, Clone, Default)]
pub struct Markdown {
    parser: Parser,
    transformer: Transformer,
    stringifier: Stringifier,
}

impl Markdown {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parser(&mut self, parser: Parser) -> &mut Self {
        self.parser = parser;
        self
    }

    pub fn transformer(&mut self, transformer: Transformer) -> &mut Self {
        self.transformer = transformer;
        self
    }

    pub fn stringifier(&mut self, stringifier: Stringifier) -> &mut Self {
        self.stringifier = stringifier;
        self
    }
}

impl Markdown {
    pub fn execute(&self, input: &str) -> String {
        let tokens = lex(input);
        let tree = self.parser.parse(input, tokens);
        let document = self.transformer.transform(tree);
        self.stringifier.stringify(document)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown() {
        let input = concat![
            "# Hello World\n\n",
            "This is **TEST**\n\n",
            "## Goodbye\n",
            "I'm happy\n\n",
            "See you\n",
            "again\n"
        ];

        let output = Markdown::default().execute(input);

        assert_eq!(
            &output,
            "<h1>Hello World</h1><p>This is <strong>TEST</strong></p><h2>Goodbye<br>I'm happy</h2><p>See you<br>again</p>"
        );
    }

    #[test]
    fn test_markdown2() {
        let input = concat![
            "- Hello\n",
            "- World\n",
            "  - Change the **world**\n",
            "  - Great!\n",
            "    1. Yeah\n",
            "    1. Wryyyyy\n",
            "- End of the world\n"
        ];

        let output = Markdown::default().execute(input);

        assert_eq!(
            &output,
            "<ul><li>Hello</li><li>World<ul><li>Change the <strong>world</strong></li><li>Great!<ol><li>Yeah</li><li>Wryyyyy</li></ol></li></ul></li><li>End of the world</li></ul>")
    }

    #[test]
    fn test_markdown3() {
        let input = concat![
            "- AAA\n",
            "- BBB\n",
            "- CCC\n",
            "\n",
            "Happy\n",
            "\n",
            "> Ok!\n",
            "> Good!\n",
            ">\n",
            "> - Yeah\n",
            "> - Wryyyyy\n",
            ">   - Change the **world**\n",
            ">\n",
            "End of the world\n",
        ];

        let output = Markdown::default().execute(input);

        assert_eq!(
            &output,
            "<ul><li>AAA</li><li>BBB</li><li>CCC</li></ul><p>Happy</p><blockquote><p>Ok!<br>Good!</p><ul><li>Yeah</li><li>Wryyyyy<ul><li>Change the <strong>world</strong></li></ul></li></ul></blockquote><p>End of the world</p>")
    }
}
