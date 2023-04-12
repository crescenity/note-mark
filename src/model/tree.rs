use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownTree<'a> {
    pub root: BlockTree<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockTree<'a> {
    pub root: Vec<BlockItem<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockItem<'a> {
    Paragraph(InlineTree<'a>),
    Headline(u8, InlineTree<'a>),
    BulletList(ListTree<'a>),
    OrderedList(ListTree<'a>),
    BlockQuote(BlockTree<'a>),
    Container(Vec<String>, BlockTree<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListTree<'a> {
    pub root: Vec<ListItem<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem<'a> {
    pub name: InlineTree<'a>,
    pub children: Vec<BlockItem<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineTree<'a> {
    pub root: Vec<InlineItem<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InlineItem<'a> {
    Text(Cow<'a, str>),
    Italic(InlineTree<'a>),
    Strong(InlineTree<'a>),
    Break,
}
