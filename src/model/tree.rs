//! The tree structure of the parsed markdown document.

use std::borrow::Cow;

/// The struct to represent a root markdown document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownTree<'a> {
    pub root: BlockTree<'a>,
}

/// The struct to represent a block tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockTree<'a> {
    pub root: Vec<BlockItem<'a>>,
}

/// The enum to represent a block item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockItem<'a> {
    Paragraph(InlineTree<'a>),
    Headline(u8, InlineTree<'a>),
    BulletList(ListTree<'a>),
    OrderedList(ListTree<'a>),
    BlockQuote(BlockTree<'a>),
    Container(Vec<String>, BlockTree<'a>),
}

/// The struct to represent a list tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListTree<'a> {
    pub root: Vec<ListItem<'a>>,
}

/// The struct to represent a list item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem<'a> {
    /// A label of the list item.
    pub name: InlineTree<'a>,
    /// Children of the list item.
    pub children: Vec<BlockItem<'a>>,
}

/// The struct to represent an inline tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineTree<'a> {
    pub root: Vec<InlineItem<'a>>,
}

/// The enum to represent an inline item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InlineItem<'a> {
    Text(Cow<'a, str>),
    Italic(InlineTree<'a>),
    Strong(InlineTree<'a>),
    Break,
}
