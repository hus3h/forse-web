use std::{collections::HashMap, vec};

use utils::attirbutes_to_inline_html;
use utils::parse_html_selector;

mod utils;

pub enum Node {
    Tag(TagNode),
    Text(RawTextNode),
    Html(RawTextNode),
    None,
}

impl Node {
    pub fn to_html(&self) -> String {
        match self {
            Self::Tag(elem) => {
                // todo: consider elements with no closing tags like input
                let inner: String = elem.children.iter().map(|item| item.to_html()).collect();
                if let Some(properties) = &elem.properties {
                    let tag = &properties.tag;
                    let mut attributes = attirbutes_to_inline_html(&properties.attributes);
                    if attributes.len() > 0 {
                        attributes = " ".to_owned() + &attributes;
                    }
                    format!("<{tag}{attributes}>{inner}</{tag}>")
                } else {
                    inner
                }
            }
            Self::Text(text) => text.content.to_owned(), // todo: escape text
            Self::Html(html) => html.content.to_owned(),
            Self::None => String::from(""),
        }
    }
}

pub struct RawTextNode {
    content: String,
}

impl RawTextNode {
    pub fn from(content: impl ToString) -> Self {
        RawTextNode {
            content: content.to_string(),
        }
    }
}

pub struct TagNode {
    properties: Option<NodeProperties>,
    children: Vec<Node>,
}

pub trait ToNode {
    fn to_node(self) -> Node;
}

#[derive(Debug)]
pub struct NodeProperties {
    tag: String,
    attributes: HashMap<String, String>,
}

pub fn elem(selector: &str, children: impl ToNode) -> Node {
    let mut properties = parse_html_selector(selector);
    if properties.tag.len() == 0 {
        properties.tag = "div".to_string();
    }
    Node::Tag(TagNode {
        properties: Some(properties),
        children: vec![children.to_node()],
    })
}

impl ToNode for Node {
    fn to_node(self) -> Node {
        self
    }
}

impl<T: ToNode> ToNode for Vec<T> {
    fn to_node(self) -> Node {
        Node::Tag(TagNode {
            properties: None,
            children: self.into_iter().map(|item| item.to_node()).collect(),
        })
    }
}

impl ToNode for Option<Node> {
    fn to_node(self) -> Node {
        if let Some(v) = self {
            v.to_node()
        } else {
            Node::None
        }
    }
}

impl ToNode for &str {
    fn to_node(self) -> Node {
        Node::Text(RawTextNode::from(self))
    }
}
