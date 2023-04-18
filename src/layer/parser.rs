//! Parser of tokens.
//!
//! This module provides a parser of tokens. The parser is implemented as a
//! recursive descent parser.

use crate::model::{token::*, tree::*};
use config::*;

/// Parser of tokens.
///
/// This struct contains configurations for parsing. These configurations are
/// for supporting various markdown syntax.
///
/// # Example
///
/// ```
/// use note_mark::prelude::*;
///
/// let parser = Parser::default().headline_ending(HeadlineEnding::SoftBreak);
///
/// let markdown = Markdown::default().parser(parser);
///
/// let html = markdown.execute("# Hello, world!\nThis is a new line.");
///
/// assert_eq!(html, "<h1>Hello, world!</h1><p>This is a new line.</p>");
///
/// let parser = Parser::default().headline_ending(HeadlineEnding::HardBreak);
///
/// let markdown = Markdown::default().parser(parser);
///
/// let html = markdown.execute("# Hello, world!\nThis is a new line.");
///
/// assert_eq!(html, "<h1>Hello, world!<br>This is a new line.</h1>");
/// ```
#[derive(Debug, Clone)]
pub struct Parser {
    /// The end of paragraph is decided by at liest two consecutive line breaks.
    /// This determines whether to treat the previous sentence as a paragraph if
    /// the next line is another block element.
    pub paragraph_ending: ParagraphEnding,
    /// This determines whether to allow a line break in a headline.
    pub headline_ending: HeadlineEnding,
    /// This determines whether to make the indent rule of list strict or loose.
    pub list_indent_rule: IndentRule,
    /// This determines whether to make the indent style of list space, tab, or
    /// both.
    pub list_indent_style: IndentStyle,
}

pub mod config {
    //! Configurations for parsing.
    //!
    //! This module provides configurations for parsing. The configurations are
    //! used in [Parser](super::Parser).

    /// Ending of paragraph.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ParagraphEnding {
        AllowSoftBreak,
        HardBreak,
    }

    /// Ending of headline.
    #[allow(clippy::enum_variant_names)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum HeadlineEnding {
        SoftBreak,
        AllowSoftBreak,
        HardBreak,
    }

    /// Indent rule of list.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum IndentRule {
        Strict,
        Loose,
    }

    /// Indent style of list.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum IndentStyle {
        Space(u8),
        Tab,
        Both,
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            paragraph_ending: ParagraphEnding::HardBreak,
            headline_ending: HeadlineEnding::HardBreak,
            list_indent_rule: IndentRule::Strict,
            list_indent_style: IndentStyle::Space(2),
        }
    }
}

impl Parser {
    /// Create a new parser.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set ending of paragraph.
    ///
    /// # Example
    ///
    /// ```
    /// use note_mark::prelude::*;
    ///
    /// let parser = Parser::default().paragraph_ending(ParagraphEnding::AllowSoftBreak);
    ///
    /// let markdown = Markdown::default().parser(parser);
    ///
    /// let html = markdown.execute("Hello, world!\n# This is a new headline.");
    ///
    /// assert_eq!(html, "<p>Hello, world!</p><h1>This is a new headline.</h1>");
    ///
    /// let parser = Parser::default().paragraph_ending(ParagraphEnding::HardBreak);
    ///
    /// let markdown = Markdown::default().parser(parser);
    ///
    /// let html = markdown.execute("Hello, world!\n# This is a new headline.");
    ///
    /// assert_eq!(html, "<p>Hello, world!<br># This is a new headline.</p>");
    /// ```
    pub fn paragraph_ending(mut self, ending: ParagraphEnding) -> Self {
        self.paragraph_ending = ending;

        self
    }

    /// Set ending of headline.
    ///
    /// # Example
    ///
    /// ```
    /// use note_mark::prelude::*;
    ///
    /// let parser = Parser::default().headline_ending(HeadlineEnding::SoftBreak);
    ///
    /// let markdown = Markdown::default().parser(parser);
    ///
    /// let html = markdown.execute("# Hello, world!\nThis is a new line.");
    ///
    /// assert_eq!(html, "<h1>Hello, world!</h1><p>This is a new line.</p>");
    ///
    /// let parser = Parser::default().headline_ending(HeadlineEnding::HardBreak);
    ///
    /// let markdown = Markdown::default().parser(parser);
    ///
    /// let html = markdown.execute("# Hello, world!\nThis is a new line.");
    ///
    /// assert_eq!(html, "<h1>Hello, world!<br>This is a new line.</h1>");
    /// ```
    pub fn headline_ending(mut self, ending: HeadlineEnding) -> Self {
        self.headline_ending = ending;

        self
    }

    /// Set indent rule of list.
    ///
    /// **This config did not work correctly.**
    pub fn list_indent_rule(mut self, rule: IndentRule) -> Self {
        self.list_indent_rule = rule;

        self
    }

    /// Set indent style of list.
    ///
    /// # Example
    ///
    /// ```
    /// use note_mark::prelude::*;
    ///
    /// let parser = Parser::default().list_indent_style(IndentStyle::Space(2));
    ///
    /// let markdown = Markdown::default().parser(parser);
    ///
    /// let html = markdown.execute("- Hello, world!\n  - This is a new line.");
    ///
    /// assert_eq!(html, "<ul><li>Hello, world!<ul><li>This is a new line.</li></ul></li></ul>");
    ///
    /// let parser = Parser::default().list_indent_style(IndentStyle::Tab);
    ///
    /// let markdown = Markdown::default().parser(parser);
    ///
    /// let html = markdown.execute("- Hello, world!\n\t- This is a new line.");
    ///
    /// assert_eq!(html, "<ul><li>Hello, world!<ul><li>This is a new line.</li></ul></li></ul>");
    /// ```
    pub fn list_indent_style(mut self, style: IndentStyle) -> Self {
        self.list_indent_style = style;

        self
    }

    /// Set all indent style.
    ///
    /// Currently, this setting is only for list.
    pub fn indent_style(mut self, style: IndentStyle) -> Self {
        self.list_indent_style = style;

        self
    }

    /// Parse tokens to markdown tree.
    pub fn parse<'a>(
        &self,
        input: &'a str,
        tokens: impl Iterator<Item = Token>,
    ) -> MarkdownTree<'a> {
        Executor::with_config(input, self.clone()).parse(tokens.collect::<Vec<Token>>())
    }
}

/// Executor of parser.
struct Executor<'a> {
    input: &'a str,
    config: Parser,
}

/// # Functions for constructing Executor and parsing tokens.
impl<'a> Executor<'a> {
    /// Create a new executor.
    #[allow(dead_code)]
    fn new(input: &'a str) -> Self {
        Self {
            input,
            config: Parser::new(),
        }
    }

    /// Create a new executor with config.
    fn with_config(input: &'a str, config: Parser) -> Self {
        Self { input, config }
    }

    /// Parse tokens to markdown tree.
    fn parse(&self, tokens: Vec<Token>) -> MarkdownTree<'a> {
        self.markdown_tree(&tokens)
    }
}

/// # Utility functions for parsing.
#[allow(dead_code)]
impl<'a, 'b> Executor<'a> {
    /// Trim tokens from start.
    fn trim_start(tokens: &'b [Token], kind: TokenKind) -> &'b [Token] {
        let mut temp = tokens;

        loop {
            if temp.is_empty() {
                break;
            }

            if temp[0].kind == kind {
                temp = &temp[1..];
            } else {
                break;
            }
        }

        temp
    }

    /// Trim tokens from end.
    fn trim_end(tokens: &'b [Token], kind: TokenKind) -> &'b [Token] {
        let mut temp = tokens;

        loop {
            if temp.is_empty() {
                break;
            }

            if temp[temp.len() - 1].kind == kind {
                temp = &temp[..temp.len() - 1];
            } else {
                break;
            }
        }

        temp
    }

    /// Trim tokens from start and end.
    fn trim(tokens: &'b [Token], kind: TokenKind) -> &'b [Token] {
        Self::trim_start(Self::trim_end(tokens, kind), kind)
    }

    /// Trim white spaces from start.
    fn trim_white_spaces(tokens: &'b [Token]) -> &'b [Token] {
        let mut rest = tokens;

        loop {
            let mut new_rest = Self::trim_start(rest, TokenKind::Space);
            new_rest = Self::trim_start(new_rest, TokenKind::Tab);

            if new_rest.len() == rest.len() {
                break rest;
            }

            rest = new_rest;
        }
    }

    /// Get a line of tokens.
    ///
    /// # Arguments
    ///
    /// * `trim` - Trim white spaces of rest tokens from start.
    fn get_line(tokens: &'b [Token], trim: bool) -> (&'b [Token], &'b [Token]) {
        if let Some(index) = tokens
            .iter()
            .position(|token| token.kind == TokenKind::Break)
        {
            if trim {
                (
                    &tokens[..index],
                    Self::trim_start(&tokens[index + 1..], TokenKind::Break),
                )
            } else {
                (&tokens[..index], &tokens[index + 1..])
            }
        } else {
            (Self::trim_end(tokens, TokenKind::Break), &[])
        }
    }

    /// Get a paragraph of tokens.
    fn get_paragraph(tokens: &'b [Token]) -> (&'b [Token], &'b [Token]) {
        if let Some(index) = tokens.windows(2).position(|tokens| {
            tokens[0].kind == TokenKind::Break && tokens[1].kind == TokenKind::Break
        }) {
            (
                &tokens[..index],
                Self::trim_start(&tokens[index + 2..], TokenKind::Break),
            )
        } else {
            (Self::trim_end(tokens, TokenKind::Break), &[])
        }
    }

    /// Count indent level.
    ///
    /// # Returns
    ///
    /// (level: u32, remainder: u32)
    fn indent_level(tokens: &[Token], style: IndentStyle) -> (u32, u32) {
        match style {
            IndentStyle::Space(n) => {
                let mut level = 0;

                for token in tokens {
                    match token.kind {
                        TokenKind::Space => level += 1,
                        _ => break,
                    }
                }

                ((level / n) as u32, (level % n) as u32)
            }
            IndentStyle::Tab => {
                let mut level = 0;

                for token in tokens {
                    match token.kind {
                        TokenKind::Tab => level += 1,
                        _ => break,
                    }
                }

                (level, 0)
            }
            IndentStyle::Both => {
                let mut level = 0;

                for token in tokens {
                    match token.kind {
                        TokenKind::Space => level += 1,
                        TokenKind::Tab => level += 2,
                        _ => break,
                    }
                }

                (level / 2, level % 2)
            }
        }
    }

    /// Reduce indent level.
    ///
    /// # Arguments
    ///
    /// * `format` - If true, remove remainder.
    fn reduce_indent(tokens: &[Token], style: IndentStyle, format: bool) -> Vec<Token> {
        let mut output = vec![];

        let mut rest = tokens;

        loop {
            let (line, new_rest) = Self::get_line(rest, false);

            if rest.is_empty() {
                break;
            }

            let (level, remainder) = Self::indent_level(line, style);

            if level == 0 {
                output.extend_from_slice(line);
            } else {
                match style {
                    IndentStyle::Space(n) => {
                        if format {
                            output.extend_from_slice(&line[(n as usize) + (remainder as usize)..]);
                        } else {
                            output.extend_from_slice(&line[n as usize..]);
                        }
                    }
                    IndentStyle::Tab => {
                        output.extend_from_slice(&line[1..]);
                    }
                    IndentStyle::Both => {
                        if line[0].kind == TokenKind::Space {
                            if format {
                                output.extend_from_slice(&line[2 + (remainder as usize)..]);
                            } else {
                                output.extend_from_slice(&line[2..]);
                            }
                        } else {
                            output.extend_from_slice(&line[1..]);
                        }
                    }
                }
            }

            if let Some(break_token) = rest.get(line.len()) {
                output.push(*break_token);
            }

            rest = new_rest;
        }

        output
    }

    fn align_indent(tokens: &'b [Token], style: IndentStyle, rule: IndentRule) -> &'b [Token] {
        match rule {
            IndentRule::Strict => tokens,
            IndentRule::Loose => {
                let (_, remainder) = Self::indent_level(tokens, style);

                &tokens[remainder as usize..]
            }
        }
    }
}

/// # Fuctions for building block tree.
impl<'a, 'b> Executor<'a> {
    /// Parse tokens to markdown tree.
    fn markdown_tree(&self, tokens: &'b [Token]) -> MarkdownTree<'a> {
        MarkdownTree {
            root: self.block_tree(tokens),
        }
    }

    /// Parse tokens to block tree.
    fn block_tree(&self, tokens: &'b [Token]) -> BlockTree<'a> {
        let mut tree = BlockTree { root: vec![] };

        let mut rest = tokens;

        'root: while !rest.is_empty() {
            for f in [Self::not_paragraph, Self::paragraph] {
                if let Some((item, new_rest)) = f(self, rest) {
                    tree.root.push(item);
                    rest = new_rest;
                    continue 'root;
                }
            }
        }

        tree
    }

    /// Parse tokens to paragraph item.
    fn paragraph(&self, tokens: &'b [Token]) -> Option<(BlockItem<'a>, &'b [Token])> {
        match self.config.paragraph_ending {
            ParagraphEnding::HardBreak => {
                let (input, rest) = Self::get_paragraph(tokens);

                Some((BlockItem::Paragraph(self.inline_tree(input)), rest))
            }
            ParagraphEnding::AllowSoftBreak => {
                let (input, rest) = self.get_until_maybe_block_item(tokens);

                Some((BlockItem::Paragraph(self.inline_tree(input)), rest))
            }
        }
    }

    /// Parse tokens to not paragraph item.
    fn not_paragraph(&self, tokens: &'b [Token]) -> Option<(BlockItem<'a>, &'b [Token])> {
        for f in [
            Self::headline,
            Self::bullet_list,
            Self::ordered_list,
            Self::blockquote,
        ] {
            if let Some((item, rest)) = f(self, tokens) {
                return Some((item, rest));
            }
        }

        None
    }

    /// Parse tokens to headline item.
    fn headline(&self, tokens: &'b [Token]) -> Option<(BlockItem<'a>, &'b [Token])> {
        let tokens = Self::trim_white_spaces(tokens);

        let mut level = 0;

        for i in 0..7 {
            if let Some(token) = tokens.get(i) {
                match token.kind {
                    TokenKind::Pound => continue,
                    TokenKind::Space => {
                        level = i;
                        break;
                    }
                    _ => return None,
                }
            }
        }

        if level == 0 {
            return None;
        }

        let content = Self::trim_start(&tokens[level..], TokenKind::Space);

        match self.config.headline_ending {
            HeadlineEnding::SoftBreak => {
                let (input, rest) = Self::get_line(content, true);

                Some((
                    BlockItem::Headline(level as u8, self.inline_tree(input)),
                    rest,
                ))
            }
            HeadlineEnding::AllowSoftBreak => {
                let (input, rest) = self.get_until_maybe_block_item(content);

                Some((
                    BlockItem::Headline(level as u8, self.inline_tree(input)),
                    rest,
                ))
            }
            HeadlineEnding::HardBreak => {
                let (input, rest) = Self::get_paragraph(content);

                Some((
                    BlockItem::Headline(level as u8, self.inline_tree(input)),
                    rest,
                ))
            }
        }
    }

    /// Parse tokens to bullet list item.
    fn bullet_list(&self, tokens: &'b [Token]) -> Option<(BlockItem<'a>, &'b [Token])> {
        let mut tree = ListTree { root: vec![] };

        let mut rest = tokens;

        let input2 = Self::align_indent(
            tokens,
            self.config.list_indent_style,
            self.config.list_indent_rule,
        );

        if input2.get(0)?.kind != TokenKind::Hyphen || input2.get(1)?.kind != TokenKind::Space {
            return None;
        }

        while !rest.is_empty() {
            let input3 = Self::align_indent(
                rest,
                self.config.list_indent_style,
                self.config.list_indent_rule,
            );

            if input3.get(0)?.kind != TokenKind::Hyphen || input3.get(1)?.kind != TokenKind::Space {
                break;
            }

            let (input, new_rest) = self.get_until_maybe_block_item(&rest[2..]);

            if input.is_empty() {
                break;
            }

            tree.root.push(self.list_item(input));

            rest = new_rest;
        }

        Some((BlockItem::BulletList(tree), rest))
    }

    fn ordered_list(&self, tokens: &'b [Token]) -> Option<(BlockItem<'a>, &'b [Token])> {
        let mut tree = ListTree { root: vec![] };

        let mut rest = tokens;

        let input2 = Self::align_indent(
            tokens,
            self.config.list_indent_style,
            self.config.list_indent_rule,
        );

        if input2.get(0)?.kind != TokenKind::Text
            || input2.get(1)?.kind != TokenKind::Dot
            || input2.get(2)?.kind != TokenKind::Space
        {
            return None;
        }

        if !self.input[tokens[0].range()]
            .chars()
            .all(|c| c.is_ascii_digit())
        {
            return None;
        }

        while !rest.is_empty() {
            let input3 = Self::align_indent(
                rest,
                self.config.list_indent_style,
                self.config.list_indent_rule,
            );

            if input3.get(0)?.kind != TokenKind::Text
                || input3.get(1)?.kind != TokenKind::Dot
                || input3.get(2)?.kind != TokenKind::Space
            {
                break;
            }

            if !self.input[input3[0].range()]
                .chars()
                .all(|c| c.is_ascii_digit())
            {
                break;
            }

            let (input, new_rest) = self.get_until_maybe_block_item(&rest[3..]);

            if input.is_empty() {
                break;
            }

            tree.root.push(self.list_item(input));

            rest = new_rest;
        }

        Some((BlockItem::OrderedList(tree), rest))
    }

    fn list_item(&self, tokens: &'b [Token]) -> ListItem<'a> {
        let (name, children_rest) = {
            let mut this_rest = tokens;

            let mut name = InlineTree { root: vec![] };

            while !this_rest.is_empty() {
                let (input, rest) = Self::get_line(this_rest, false);

                if input.is_empty() {
                    break;
                }

                if Self::indent_level(input, self.config.list_indent_style).0 != 0 {
                    break;
                }

                name.root.append(&mut self.inline_tree(input).root);

                name.root.push(InlineItem::Break);

                this_rest = rest;
            }

            name.root.pop();

            (name, this_rest)
        };

        let tokens = Self::reduce_indent(children_rest, self.config.list_indent_style, true);

        ListItem {
            name,
            children: self.block_tree(&tokens).root,
        }
    }

    fn blockquote(&self, tokens: &'b [Token]) -> Option<(BlockItem<'a>, &'b [Token])> {
        if tokens.get(0)?.kind != TokenKind::Gt {
            return None;
        }

        let mut rest = tokens;

        let mut indented_tokens = vec![];

        while !rest.is_empty() {
            if rest.get(0)?.kind != TokenKind::Gt {
                break;
            }

            let (input, new_rest) = Self::get_line(&rest[1..], false);

            let input2 = if self.maybe_block_item(input, true) {
                Self::align_indent(input, IndentStyle::Space(2), IndentRule::Loose)
            } else {
                Self::trim_start(input, TokenKind::Space)
            };

            indented_tokens.extend_from_slice(input2);

            if let Some(token) = rest.get(1 + input.len()) {
                indented_tokens.push(*token);
            }

            rest = new_rest;
        }

        let tree = self.block_tree(&indented_tokens);

        Some((BlockItem::BlockQuote(tree), rest))
    }

    /// Judge if tokens is maybe block item.
    fn maybe_block_item(&self, tokens: &[Token], trim: bool) -> bool {
        let tokens = if trim {
            Self::trim_white_spaces(tokens)
        } else {
            tokens
        };

        if self.headline(tokens).is_some() {
            return true;
        }

        if tokens.is_empty() {
            return false;
        }

        if tokens[0].kind == TokenKind::Gt {
            return true;
        }

        if tokens.len() < 2 {
            return false;
        }

        if tokens[0].kind == TokenKind::Hyphen && tokens[1].kind == TokenKind::Space {
            return true;
        }

        if tokens.len() < 3 {
            return false;
        }

        if (tokens[0].kind == TokenKind::Text
            && tokens[1].kind == TokenKind::Dot
            && tokens[2].kind == TokenKind::Space)
            && self.input[tokens[0].range()]
                .chars()
                .all(|c| c.is_ascii_digit())
        {
            return true;
        }

        false
    }

    /// Get tokens until maybe block item.
    fn get_until_maybe_block_item(&self, tokens: &'b [Token]) -> (&'b [Token], &'b [Token]) {
        let mut iter = Self::trim_end(tokens, TokenKind::Break).iter().enumerate();

        let (front, back) = loop {
            if let Some((index, _)) = iter.find(|(_, token)| token.kind == TokenKind::Break) {
                if self.maybe_block_item(&tokens[index + 1..], false) {
                    break (&tokens[..index], &tokens[index + 1..]);
                } else if tokens[index].kind == TokenKind::Break
                    && tokens[index + 1].kind == TokenKind::Break
                {
                    break (&tokens[..index], &tokens[index + 2..]);
                }
            } else {
                break (tokens, &[]);
            }
        };

        (
            Self::trim_end(front, TokenKind::Break),
            Self::trim_start(back, TokenKind::Break),
        )
    }
}

/// # Functions for building inline tree.
impl<'a, 'b> Executor<'a> {
    /// Parse tokens to inline tree.
    ///
    /// This function parses all tokens to inline tree.
    /// So confirm that tokens does not include block items.
    fn inline_tree(&self, tokens: &[Token]) -> InlineTree<'a> {
        let mut tree = InlineTree { root: vec![] };

        let mut rest = tokens;

        'root: while !rest.is_empty() {
            for f in &[Self::strong, Self::italic, Self::r#break] {
                if let Some((item, new_rest)) = f(self, rest) {
                    tree.root.push(item);
                    rest = new_rest;
                    continue 'root;
                }
            }

            if let Some(InlineItem::Text(text)) = tree.root.last_mut() {
                *text += &self.input[rest[0].range()];
                rest = &rest[1..];
                continue;
            } else {
                tree.root
                    .push(InlineItem::Text(self.input[rest[0].range()].into()));
                rest = &rest[1..];
                continue;
            }
        }

        tree
    }

    /// Parse tokens to italic item.
    fn italic(&self, tokens: &'b [Token]) -> Option<(InlineItem<'a>, &'b [Token])> {
        if tokens[0].kind != TokenKind::Star {
            return None;
        }

        let (index, _) = tokens
            .iter()
            .enumerate()
            .skip(1)
            .find(|(_, token)| token.kind == TokenKind::Star)?;

        let tree = self.inline_tree(&tokens[1..index]);

        Some((InlineItem::Italic(tree), &tokens[index + 1..]))
    }

    /// Parse tokens to strong item.
    fn strong(&self, tokens: &'b [Token]) -> Option<(InlineItem<'a>, &'b [Token])> {
        if tokens[0].kind != TokenKind::Star || tokens.get(1)?.kind != TokenKind::Star {
            return None;
        }

        let (index, _) = tokens
            .windows(2)
            .enumerate()
            .skip(1)
            .find(|(_, t)| t[0].kind == TokenKind::Star && t[1].kind == TokenKind::Star)?;

        let tree = self.inline_tree(&tokens[2..index]);

        Some((InlineItem::Strong(tree), &tokens[index + 2..]))
    }

    /// Parse tokens to break item.
    fn r#break(&self, tokens: &'b [Token]) -> Option<(InlineItem<'a>, &'b [Token])> {
        if tokens[0].kind != TokenKind::Break {
            return None;
        }

        Some((InlineItem::Break, &tokens[1..]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layer::lexer::lex;

    fn lex_to_vec(input: &str) -> Vec<Token> {
        lex(input).collect()
    }

    #[test]
    fn test_parse() {
        let input = "# Hello *World*!\n\nparagraph\n\n";

        let tokens = lex(input);

        let tree = Parser::new().parse(input, tokens);

        assert_eq!(
            tree,
            MarkdownTree {
                root: BlockTree {
                    root: vec![
                        BlockItem::Headline(
                            1,
                            InlineTree {
                                root: vec![
                                    InlineItem::Text("Hello ".into()),
                                    InlineItem::Italic(InlineTree {
                                        root: vec![InlineItem::Text("World".into())]
                                    }),
                                    InlineItem::Text("!".into()),
                                ]
                            }
                        ),
                        BlockItem::Paragraph(InlineTree {
                            root: vec![InlineItem::Text("paragraph".into())]
                        }),
                    ]
                }
            }
        );
    }

    #[test]
    fn test_reduce_indent() {
        let input = "  # Hello *World*!\n\nparagraph\n\n";
        let tokens = lex_to_vec(input);

        let result = Executor::reduce_indent(&tokens, IndentStyle::Space(2), true)
            .into_iter()
            .map(|token| token.kind)
            .collect::<Vec<_>>();

        let expected = "# Hello *World*!\n\nparagraph\n\n";
        let expected_tokens = lex(expected)
            .into_iter()
            .map(|token| token.kind)
            .collect::<Vec<_>>();

        assert_eq!(result, expected_tokens);
    }

    #[test]
    fn test_block_tree() {
        let input = "# Hello *World*!\n\nparagraph\n\n";
        let tokens = lex_to_vec(input);
        let parser = Executor::new(input);

        let tree = parser.block_tree(&tokens);

        assert_eq!(
            tree,
            BlockTree {
                root: vec![
                    BlockItem::Headline(
                        1,
                        InlineTree {
                            root: vec![
                                InlineItem::Text("Hello ".into()),
                                InlineItem::Italic(InlineTree {
                                    root: vec![InlineItem::Text("World".into())]
                                }),
                                InlineItem::Text("!".into()),
                            ]
                        }
                    ),
                    BlockItem::Paragraph(InlineTree {
                        root: vec![InlineItem::Text("paragraph".into())]
                    }),
                ]
            }
        );
    }

    #[test]
    fn test_paragraph() {
        let input = "Hello *World*!\n\n";
        let tokens = lex_to_vec(input);
        let parser = Executor::new(input);

        let (item, rest) = parser.paragraph(&tokens).unwrap();

        assert_eq!(
            item,
            BlockItem::Paragraph(InlineTree {
                root: vec![
                    InlineItem::Text("Hello ".into()),
                    InlineItem::Italic(InlineTree {
                        root: vec![InlineItem::Text("World".into())]
                    }),
                    InlineItem::Text("!".into()),
                ]
            })
        );
        assert_eq!(rest.len(), 0);

        let input = "Hello\n";

        let tokens = lex_to_vec(input);
        let parser = Executor::new(input);

        let (item, rest) = parser.paragraph(&tokens).unwrap();

        assert_eq!(
            item,
            BlockItem::Paragraph(InlineTree {
                root: vec![InlineItem::Text("Hello".into())]
            })
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_paragraph_before_not_paragraph() {
        let input = "Hello *World*!\n# Hello\n";
        let tokens = lex_to_vec(input);
        let parser = Executor::with_config(
            input,
            Parser::new().paragraph_ending(ParagraphEnding::AllowSoftBreak),
        );

        let (item, rest) = parser.paragraph(&tokens).unwrap();

        assert_eq!(
            item,
            BlockItem::Paragraph(InlineTree {
                root: vec![
                    InlineItem::Text("Hello ".into()),
                    InlineItem::Italic(InlineTree {
                        root: vec![InlineItem::Text("World".into())]
                    }),
                    InlineItem::Text("!".into()),
                ]
            })
        );

        assert_eq!(rest.len(), 4);
    }

    #[test]
    fn test_headline() {
        let input = "###  Hello *World*!\n\n";
        let tokens = lex_to_vec(input);
        let parser = Executor::new(input);

        let (item, rest) = parser.headline(&tokens).unwrap();

        assert_eq!(
            item,
            BlockItem::Headline(
                3,
                InlineTree {
                    root: vec![
                        InlineItem::Text("Hello ".into()),
                        InlineItem::Italic(InlineTree {
                            root: vec![InlineItem::Text("World".into())]
                        }),
                        InlineItem::Text("!".into()),
                    ]
                }
            )
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_headline2() {
        let input = "# Hello World!\n# Goodbye\n";
        let tokens = lex_to_vec(input);
        let parser = Executor::new(input);

        let (item, _) = parser.headline(&tokens).unwrap();

        assert_ne!(
            item,
            BlockItem::Headline(
                1,
                InlineTree {
                    root: vec![InlineItem::Text("Hello World!".into())]
                }
            )
        );

        let parser = Executor::with_config(
            input,
            Parser::default().headline_ending(HeadlineEnding::AllowSoftBreak),
        );

        let (item, _) = parser.headline(&tokens).unwrap();

        assert_eq!(
            item,
            BlockItem::Headline(
                1,
                InlineTree {
                    root: vec![InlineItem::Text("Hello World!".into())]
                }
            )
        );
    }

    #[test]
    fn test_bullet_list() {
        let input = "- Hello *World*!\n- Hello *World*!\n\n";
        let tokens = lex_to_vec(input);
        let parser = Executor::new(input);

        let (item, rest) = parser.bullet_list(&tokens).unwrap();

        assert_eq!(
            item,
            BlockItem::BulletList(ListTree {
                root: vec![
                    ListItem {
                        name: InlineTree {
                            root: vec![
                                InlineItem::Text("Hello ".into()),
                                InlineItem::Italic(InlineTree {
                                    root: vec![InlineItem::Text("World".into())]
                                }),
                                InlineItem::Text("!".into()),
                            ]
                        },
                        children: vec![]
                    },
                    ListItem {
                        name: InlineTree {
                            root: vec![
                                InlineItem::Text("Hello ".into()),
                                InlineItem::Italic(InlineTree {
                                    root: vec![InlineItem::Text("World".into())]
                                }),
                                InlineItem::Text("!".into()),
                            ]
                        },
                        children: vec![]
                    },
                ]
            }),
        );

        assert_eq!(rest.len(), 0);

        let input = "- Hello!\n  - Fooo!\nHappy\n  - hogee!\n- Good\njobs\n# End\n";
        let tokens = lex_to_vec(input);
        let parser = Executor::new(input);

        let (item, rest) = parser.bullet_list(&tokens).unwrap();

        assert_eq!(
            item,
            BlockItem::BulletList(ListTree {
                root: vec![
                    ListItem {
                        name: InlineTree {
                            root: vec![InlineItem::Text("Hello!".into())]
                        },
                        children: vec![BlockItem::BulletList(ListTree {
                            root: vec![
                                ListItem {
                                    name: InlineTree {
                                        root: vec![
                                            InlineItem::Text("Fooo!".into()),
                                            InlineItem::Break,
                                            InlineItem::Text("Happy".into())
                                        ]
                                    },
                                    children: vec![]
                                },
                                ListItem {
                                    name: InlineTree {
                                        root: vec![InlineItem::Text("hogee!".into())]
                                    },
                                    children: vec![]
                                }
                            ]
                        }),]
                    },
                    ListItem {
                        name: InlineTree {
                            root: vec![
                                InlineItem::Text("Good".into()),
                                InlineItem::Break,
                                InlineItem::Text("jobs".into())
                            ]
                        },
                        children: vec![]
                    },
                ]
            }),
        );

        assert_eq!(rest.len(), 4);
    }

    #[test]
    fn ordered_list() {
        let input = "1. Hello!\n  1. Fooo!\nHappy\n  1. hogee!\n1. Good\njobs\n# End\n";
        let tokens = lex_to_vec(input);
        let parser = Executor::new(input);

        let (item, rest) = parser.ordered_list(&tokens).unwrap();

        assert_eq!(
            item,
            BlockItem::OrderedList(ListTree {
                root: vec![
                    ListItem {
                        name: InlineTree {
                            root: vec![InlineItem::Text("Hello!".into())]
                        },
                        children: vec![BlockItem::OrderedList(ListTree {
                            root: vec![
                                ListItem {
                                    name: InlineTree {
                                        root: vec![
                                            InlineItem::Text("Fooo!".into()),
                                            InlineItem::Break,
                                            InlineItem::Text("Happy".into())
                                        ]
                                    },
                                    children: vec![]
                                },
                                ListItem {
                                    name: InlineTree {
                                        root: vec![InlineItem::Text("hogee!".into())]
                                    },
                                    children: vec![]
                                }
                            ]
                        }),]
                    },
                    ListItem {
                        name: InlineTree {
                            root: vec![
                                InlineItem::Text("Good".into()),
                                InlineItem::Break,
                                InlineItem::Text("jobs".into())
                            ]
                        },
                        children: vec![]
                    },
                ]
            }),
        );

        assert_eq!(rest.len(), 4);
    }

    #[test]
    fn test_blockquote() {
        let input = ">Hello\n>\n>>Yeah\nHappy";
        let tokens = lex_to_vec(input);
        let parser = Executor::new(input);

        let (item, rest) = parser.blockquote(&tokens).unwrap();

        assert_eq!(
            item,
            BlockItem::BlockQuote(BlockTree {
                root: vec![
                    BlockItem::Paragraph(InlineTree {
                        root: vec![InlineItem::Text("Hello".into())]
                    }),
                    BlockItem::BlockQuote(BlockTree {
                        root: vec![BlockItem::Paragraph(InlineTree {
                            root: vec![InlineItem::Text("Yeah".into())]
                        }),]
                    }),
                ]
            })
        );

        assert_eq!(rest.len(), 1);
    }

    #[test]
    fn test_inline_tree() {
        let input = "Hello *World*!\n";
        let tokens = lex_to_vec(input);
        let parser = Executor::new(input);

        let tree = parser.inline_tree(&tokens);

        assert_eq!(
            tree,
            InlineTree {
                root: vec![
                    InlineItem::Text("Hello ".into()),
                    InlineItem::Italic(InlineTree {
                        root: vec![InlineItem::Text("World".into())]
                    }),
                    InlineItem::Text("!".into()),
                    InlineItem::Break,
                ]
            }
        );
    }

    #[test]
    fn test_italic() {
        let input = r"*Hello*";
        let tokens = lex_to_vec(input);
        let parser = Executor::new(input);

        let (item, rest) = parser.italic(&tokens).unwrap();

        assert_eq!(
            item,
            InlineItem::Italic(InlineTree {
                root: vec![InlineItem::Text("Hello".into())]
            })
        );
        assert_eq!(rest.len(), 0);

        let input = "*";
        let tokens = lex_to_vec(input);
        let parser = Executor::new(input);

        assert_eq!(parser.italic(&tokens), None);
    }

    #[test]
    fn test_strong() {
        let input = r"**Hello**";
        let tokens = lex_to_vec(input);
        let parser = Executor::new(input);

        let (item, rest) = parser.strong(&tokens).unwrap();

        assert_eq!(
            item,
            InlineItem::Strong(InlineTree {
                root: vec![InlineItem::Text("Hello".into())]
            })
        );
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_break() {
        let input = "\r\nHello";
        let tokens = lex_to_vec(input);
        let parser = Executor::new(input);

        let (item, rest) = parser.r#break(&tokens).unwrap();

        assert_eq!(item, InlineItem::Break);
        assert_eq!(rest.len(), 1);
    }
}
