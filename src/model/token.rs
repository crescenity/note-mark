#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub len: usize,
}

impl Token {
    pub fn range(&self) -> std::ops::Range<usize> {
        self.start..self.start + self.len
    }
}

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
