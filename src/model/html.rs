//! HTML document model.
//!
//! This module contains the data structures used to represent an HTML.

use std::borrow::Cow;

/// The struct to represent an root HTML document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentNode<'a> {
    pub root: Vec<Node<'a>>,
}

/// The enum to represent an HTML element tag.
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
    A,
    Strong,
    Em,
    Br,
}

impl ElementTag {
    /// Whether this tag is a block item.
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
    /// Create a new ElementTag from a headline level.
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

    /// Return headline level if this tag is a headline.
    pub fn get_headline_level(&self) -> Option<u8> {
        match self {
            Self::H1 => Some(1),
            Self::H2 => Some(2),
            Self::H3 => Some(3),
            Self::H4 => Some(4),
            Self::H5 => Some(5),
            Self::H6 => Some(6),
            _ => None,
        }
    }
}

/// The enum to represent an HTML node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node<'a> {
    Element(ElementNode<'a>),
    Text(TextNode<'a>),
}

impl Node<'_> {
    /// Whether this node is a block item.
    pub fn is_block_item(&self) -> bool {
        match self {
            Node::Element(element) => element.tag.is_block_item(),
            Node::Text(_) => false,
        }
    }
}

/// Stringify a node.
pub fn get_text(nodes: &[Node<'_>]) -> String {
    nodes
        .iter()
        .map(|node| match node {
            Node::Element(element) => get_text(&element.children),
            Node::Text(text) => text.text.to_string(),
        })
        .collect::<Vec<_>>()
        .join("")
}

/// The struct to represent an HTML element node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementNode<'a> {
    /// The tag of this element.
    pub tag: ElementTag,
    /// The id of this element.
    pub id: Vec<String>,
    /// The classes of this element.
    pub class: Vec<String>,
    /// The href of this element.
    pub href: Option<String>,
    /// The attributes of this element.
    pub attrs: Vec<(String, String)>,
    /// The children of this element.
    pub children: Vec<Node<'a>>,
}

impl Default for ElementNode<'_> {
    fn default() -> Self {
        Self {
            tag: ElementTag::Div,
            id: vec![],
            class: vec![],
            href: None,
            attrs: vec![],
            children: vec![],
        }
    }
}

/// The struct to represent an HTML text node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextNode<'a> {
    pub text: Cow<'a, str>,
}
