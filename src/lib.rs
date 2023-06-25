use std::{collections::HashMap, vec};

use utils::attirbutes_to_inline_html;
use utils::parse_elem_properties;

mod utils;

const HTML_VOID_ELEMENTS: &'static [&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr", "command", "keygen", "menuitem",
];

#[derive(Clone)]
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
                if let Some(properties) = &elem.properties {
                    let tag: &str = &properties.tag.clone();
                    let mut attributes = attirbutes_to_inline_html(&properties.attributes);
                    if attributes.len() > 0 {
                        attributes = " ".to_owned() + &attributes;
                    }
                    if !HTML_VOID_ELEMENTS.contains(&tag) {
                        let inner: String =
                            elem.children.iter().map(|item| item.to_html()).collect();
                        format!("<{tag}{attributes}>{inner}</{tag}>")
                    } else {
                        format!("<{tag}{attributes} />")
                    }
                } else {
                    elem.children.iter().map(|item| item.to_html()).collect()
                }
            }
            Self::Text(text) => text.content.to_owned(), // todo: escape text
            Self::Html(html) => html.content.to_owned(),
            Self::None => String::from(""),
        }
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct TagNode {
    properties: Option<NodeProperties>,
    children: Vec<Node>,
}

pub trait ToNode {
    fn to_node(&self) -> Node;
}

#[derive(Clone, Debug)]
pub struct NodeProperties {
    tag: String,
    attributes: HashMap<String, String>,
}

// todo: macro for this
pub fn elem(selector: &str, attributes: impl ToAttributesList, children: impl ToNode) -> Node {
    let mut properties = parse_elem_properties(selector, &attributes.to_hash_map());
    if properties.tag.len() == 0 {
        properties.tag = "div".to_string();
    }
    Node::Tag(TagNode {
        properties: Some(properties),
        children: vec![children.to_node()],
    })
}

pub trait ToAttributesList {
    fn to_hash_map(&self) -> HashMap<String, String>;
}

impl ToAttributesList for Option<HashMap<String, String>> {
    fn to_hash_map(&self) -> HashMap<String, String> {
        if let Some(v) = self {
            v.clone()
        } else {
            HashMap::new()
        }
    }
}

impl ToAttributesList for Vec<(&str, &str)> {
    fn to_hash_map(&self) -> HashMap<String, String> {
        let mut result = HashMap::new();
        for (key, value) in self {
            result.insert(key.to_string(), value.to_string());
        }
        result
    }
}

impl ToNode for Node {
    fn to_node(&self) -> Node {
        self.clone()
    }
}

impl<T: ToNode> ToNode for Vec<T> {
    fn to_node(&self) -> Node {
        Node::Tag(TagNode {
            properties: None,
            children: self.into_iter().map(|item| item.to_node()).collect(),
        })
    }
}

impl ToNode for Option<Node> {
    fn to_node(&self) -> Node {
        if let Some(v) = self {
            v.clone()
        } else {
            Node::None
        }
    }
}

impl ToNode for &str {
    fn to_node(&self) -> Node {
        Node::Text(RawTextNode::from(self))
    }
}
