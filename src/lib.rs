//! note-mark is a markdown parser. It is a library that can be used to parse
//! markdown into HTML.
//!
//! # Example
//!
//! ```
//! use note_mark::prelude::*;
//!
//! let markdown = Markdown::default();
//!
//! let html = markdown.execute("# Hello, world!");
//!
//! assert_eq!(html, "<h1>Hello, world!</h1>");
//!
//! let html = markdown.execute("# Hello, world!\n\nThis is a paragraph.");
//!
//! assert_eq!(html, "<h1>Hello, world!</h1><p>This is a paragraph.</p>");
//! ```

pub mod layer;
pub mod model;
pub mod prelude;

use layer::{
    lexer::lex, parser::Parser, stringifier::Stringifier, toc::TocMaker, transformer::Transformer,
};

/// Markdown parser and transformer.
///
/// # Example
///
/// ```
/// use note_mark::prelude::*;
///
/// let markdown = Markdown::default();
///
/// let html = markdown.execute("# Hello, world!\n\nThis is a paragraph.");
///
/// assert_eq!(html, "<h1>Hello, world!</h1><p>This is a paragraph.</p>");
/// ```
#[derive(Debug, Clone, Default)]
pub struct Markdown {
    /// Parser configuration.
    parser: Parser,
    /// Transformer configuration.
    transformer: Transformer,
    /// Stringifier configuration.
    stringifier: Stringifier,
    /// Table of contents maker configuration.
    toc_maker: TocMaker,
}

impl Markdown {
    /// Create a new `Markdown` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the parser configuration.
    pub fn parser(mut self, parser: Parser) -> Self {
        self.parser = parser;
        self
    }

    /// Set the transformer configuration.
    pub fn transformer(mut self, transformer: Transformer) -> Self {
        self.transformer = transformer;
        self
    }

    /// Set the stringifier configuration.
    pub fn stringifier(mut self, stringifier: Stringifier) -> Self {
        self.stringifier = stringifier;
        self
    }

    /// Set the table of contents maker configuration.
    pub fn toc_maker(mut self, toc_maker: TocMaker) -> Self {
        self.toc_maker = toc_maker;
        self
    }
}

impl Markdown {
    /// Execute the markdown parser.
    pub fn execute(&self, input: &str) -> String {
        let tokens = lex(input);
        let tree = self.parser.parse(input, tokens);
        let document = self.transformer.transform(tree);
        self.stringifier.stringify(document)
    }

    /// Execute the markdown parser and generate the table of contents.
    ///
    /// # Example
    ///
    /// ```
    /// use note_mark::prelude::*;
    ///
    /// let markdown = Markdown::default();
    ///
    /// let input = concat![
    ///     "# Headline1-1\n\n",
    ///     "# Headline1-2\n\n",
    ///     "## Headline2-1\n\n",
    ///     "## Headline2-2\n\n",
    ///     "# Headline1-3\n\n",
    /// ];
    ///
    /// let (html, toc) = markdown.execute_with_toc(input);
    ///
    /// assert_eq!(toc, "<ul><li><a href=\"#Headline1-1\">Headline1-1</a></li><li><a href=\"#Headline1-2\">Headline1-2</a><ul><li><a href=\"#Headline2-1\">Headline2-1</a></li><li><a href=\"#Headline2-2\">Headline2-2</a></li></ul></li><li><a href=\"#Headline1-3\">Headline1-3</a></li></ul>");
    /// ```
    /// ## Original output
    ///
    /// ```html
    /// <h1 id="Headline1-1">Headline1-1</h1>
    /// <h1 id="Headline1-2">Headline1-2</h1>
    /// <h2 id="Headline2-1">Headline2-1</h2>
    /// <h2 id="Headline2-2">Headline2-2</h2>
    /// <h1 id="Headline1-3">Headline1-3</h1>
    /// ```
    ///
    /// ## Toc output
    ///
    /// ```html
    /// <ul>
    ///     <li><a href="#Headline1-1">Headline1-1</a></li>
    ///     <li>
    ///         <a href="#Headline1-2">Headline1-2</a>
    ///         <ul>
    ///             <li><a href="#Headline2-1">Headline2-1</a></li>
    ///             <li><a href="#Headline2-2">Headline2-2</a></li>
    ///         </ul>
    ///     </li>
    ///     <li><a href="#Headline1-3">Headline1-3</a></li>
    /// </ul>
    /// ```
    pub fn execute_with_toc(&self, input: &str) -> (String, String) {
        let tokens = lex(input);
        let tree = self.parser.parse(input, tokens);
        let mut document = self.transformer.transform(tree);

        let toc = self.toc_maker.make_toc(&mut document);

        (
            self.stringifier.stringify(document),
            self.stringifier.stringify(toc),
        )
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
