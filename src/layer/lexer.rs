//! Lexer for the Markdown syntax.
//!
//! This module contains the lexer for the Markdown syntax. At this stage, the
//! text is joined and non-breakable spaces are removed.

use peekmore::{PeekMore, PeekMoreIterator};
use std::iter::Peekable;

use crate::model::token::*;

/// Split a string into tokens.
///
/// This returns an iterator that yields tokens. This iterator has static
/// lifetime, though tokens refer to the input string.
pub fn lex(input: &'_ str) -> impl Iterator<Item = Token> + '_ {
    let lexer = Lexer::new(input);
    let lexer = TextJoiner::new(lexer);
    SpaceCutter::new(lexer)
}

struct Lexer<'a> {
    input: &'a str,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, cursor: 0 }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.input.char_indices().skip(self.cursor).peekable();

        let (kind, start, len) = if let Some((index, c)) = chars.next() {
            let len = c.len_utf8();

            let (kind, len) = match c {
                '#' => (TokenKind::Pound, len),
                '*' => (TokenKind::Star, len),
                ':' => (TokenKind::Colon, len),
                '`' => (TokenKind::Backquote, len),
                '>' => (TokenKind::Gt, len),
                '-' => (TokenKind::Hyphen, len),
                '|' => (TokenKind::VerticalBar, len),
                '.' => (TokenKind::Dot, len),
                '(' => (TokenKind::OpenParen, len),
                ')' => (TokenKind::CloseParen, len),
                '{' => (TokenKind::OpenBrace, len),
                '}' => (TokenKind::CloseBrace, len),
                '[' => (TokenKind::OpenBracket, len),
                ']' => (TokenKind::CloseBracket, len),
                ' ' => (TokenKind::Space, len),
                '\t' => (TokenKind::Tab, len),
                '\n' => (TokenKind::Break, len),
                '\r' => {
                    if let Some((_, c2)) = chars.next_if(|(_, c2)| c2 == &'\n') {
                        (TokenKind::Break, len + c2.len_utf8())
                    } else {
                        (TokenKind::Text, len)
                    }
                }
                '\\' => {
                    if let Some((_, c2)) = chars.next_if(|(_, c2)| {
                        matches!(
                            c2,
                            '#' | '*'
                                | ':'
                                | '`'
                                | '>'
                                | '-'
                                | '|'
                                | '.'
                                | '('
                                | ')'
                                | '{'
                                | '}'
                                | '['
                                | ']'
                                | '\\'
                        )
                    }) {
                        self.cursor += len + c2.len_utf8();
                        return Some(Token {
                            kind: TokenKind::Text,
                            start: index + len,
                            len: c2.len_utf8(),
                        });
                    } else {
                        (TokenKind::Text, len)
                    }
                }
                _ => (TokenKind::Text, len),
            };

            (kind, index, len)
        } else {
            return None;
        };

        self.cursor += len;

        Some(Token { kind, start, len })
    }
}

struct TextJoiner<T: Iterator<Item = Token>> {
    iter: Peekable<T>,
}

impl<T: Iterator<Item = Token>> TextJoiner<T> {
    fn new(iter: T) -> Self {
        Self {
            iter: iter.peekable(),
        }
    }
}

impl<T: Iterator<Item = Token>> Iterator for TextJoiner<T> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut token = self.iter.next()?;

        if token.kind == TokenKind::Text {
            while let Some(next) = self.iter.peek() {
                if next.kind == TokenKind::Text {
                    token.len += next.len;
                    self.iter.next();
                } else {
                    break;
                }
            }
        }

        Some(token)
    }
}

struct SpaceCutter<T: Iterator<Item = Token>> {
    iter: PeekMoreIterator<T>,
}

impl<T: Iterator<Item = Token>> SpaceCutter<T> {
    fn new(iter: T) -> Self {
        Self {
            iter: iter.peekmore(),
        }
    }
}

impl<T: Iterator<Item = Token>> Iterator for SpaceCutter<T> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.iter.next()?;

        use TokenKind::*;

        if token.kind == Break
            && self.iter.peek().is_some()
            && self.iter.peek().unwrap().kind != Break
        {
            for n in 0.. {
                if let Some(nth) = self.iter.peek_nth(n) {
                    match nth.kind {
                        Space | Tab => continue,
                        Break => {
                            self.iter.nth(n - 1).unwrap();
                            break;
                        }
                        _ => break,
                    }
                } else {
                    self.iter.nth(n - 1).unwrap();
                    break;
                }
            }
        }

        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("## Hello\n");

        assert_eq!(lexer.next().unwrap().kind, TokenKind::Pound);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Pound);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Space);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Text);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Text);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Text);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Text);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Text);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Break);
        assert_eq!(lexer.next(), None);

        let mut lexer = Lexer::new("\r\n");

        assert_eq!(lexer.next().unwrap().kind, TokenKind::Break);
        assert_eq!(lexer.next(), None);

        let mut lexer = Lexer::new("\r");

        assert_eq!(lexer.next().unwrap().kind, TokenKind::Text);
        assert_eq!(lexer.next(), None);

        let mut lexer = Lexer::new(r"\# Q");

        assert_eq!(lexer.next().unwrap().kind, TokenKind::Text);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Space);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Text);
        assert_eq!(lexer.next(), None);

        let mut lexer = Lexer::new("あああ");

        assert_eq!(lexer.next().unwrap().kind, TokenKind::Text);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Text);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Text);
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_text_jointer() {
        let mut lexer = TextJoiner::new(Lexer::new("## Hello Q\n"));

        assert_eq!(lexer.next().unwrap().kind, TokenKind::Pound);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Pound);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Space);
        assert_eq!(
            lexer.next().unwrap(),
            Token {
                kind: TokenKind::Text,
                start: 3,
                len: 5
            }
        );
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Space);
        assert_eq!(
            lexer.next().unwrap(),
            Token {
                kind: TokenKind::Text,
                start: 9,
                len: 1
            }
        );
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Break);
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_space_cutter() {
        let mut lexer = SpaceCutter::new(Lexer::new("ABC\n  \nDEF"));

        assert_eq!(lexer.next().unwrap().kind, TokenKind::Text);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Text);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Text);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Break);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Break);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Text);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Text);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Text);
    }
}
