use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentNode<'a> {
    pub root: Vec<Node<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementTag {
    Div,
    Span,
    P,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Ul,
    Ol,
    Li,
    Blockquote,
    Strong,
    Em,
    Br,
}

impl ElementTag {
    pub fn is_block_item(&self) -> bool {
        matches!(
            self,
            ElementTag::Div
                | ElementTag::P
                | ElementTag::Ul
                | ElementTag::Ol
                | ElementTag::Li
                | ElementTag::H1
                | ElementTag::H2
                | ElementTag::H3
                | ElementTag::H4
                | ElementTag::H5
                | ElementTag::H6
        )
    }
}

impl ElementTag {
    pub fn headline(level: u8) -> Option<Self> {
        match level {
            1 => Some(Self::H1),
            2 => Some(Self::H2),
            3 => Some(Self::H3),
            4 => Some(Self::H4),
            5 => Some(Self::H5),
            6 => Some(Self::H6),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node<'a> {
    Element(ElementNode<'a>),
    Text(TextNode<'a>),
}

impl Node<'_> {
    pub fn is_block_item(&self) -> bool {
        match self {
            Node::Element(element) => element.tag.is_block_item(),
            Node::Text(_) => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementNode<'a> {
    pub tag: ElementTag,
    pub id: Vec<String>,
    pub class: Vec<String>,
    pub children: Vec<Node<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextNode<'a> {
    pub text: Cow<'a, str>,
}
