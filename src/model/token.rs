//! Token.

/// The struct to represent a token.
///
/// Token contains the kind of token and the range of the token in the source.
/// The range is represented by the start position and the length of the token.
///
/// Note!: **This struct can live longer than the source string.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    /// The kind of the token.
    pub kind: TokenKind,
    /// The start position of the token.
    pub start: usize,
    /// The length of the token.
    pub len: usize,
}

impl Token {
    /// Get the range of the token.
    pub fn range(&self) -> std::ops::Range<usize> {
        self.start..self.start + self.len
    }
}

/// The enum to represent a token kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Text,
    /// " "
    Space,
    /// "\t"
    Tab,
    Break,
    /// "\\"
    Backslash,
    /// "#"
    Pound,
    /// "*"
    Star,
    /// ":"
    Colon,
    /// "`"
    Backquote,
    /// ">"
    Gt,
    /// "-"
    Hyphen,
    /// "|"
    VerticalBar,
    /// "."
    Dot,
    /// "("
    OpenParen,
    /// ")"
    CloseParen,
    /// "{"
    OpenBrace,
    /// "}"
    CloseBrace,
    /// "["
    OpenBracket,
    /// "]"
    CloseBracket,
}
