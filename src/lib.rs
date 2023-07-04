use std::vec;

use utils::attirbutes_to_inline_html;
use utils::attirbutes_to_json_object;
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

    // todo: make sure this follows the different hyperscript cases
    pub fn to_hyperscript(&self, function_name: &str) -> String {
        match self {
            Self::Tag(elem) => {
                if let Some(properties) = &elem.properties {
                    let tag: &str = &properties.tag.clone();
                    let mut inner_string = String::new();
                    if !HTML_VOID_ELEMENTS.contains(&tag) {
                        let inner: Vec<String> = elem
                            .children
                            .iter()
                            .map(|item| item.to_hyperscript(function_name))
                            .filter(|item| item != "")
                            .collect();
                        if inner.len() > 0 {
                            if inner.len() == 1 {
                                inner_string = ",".to_string() + &inner[0];
                            } else {
                                inner_string = ",[".to_string() + &inner.join(",") + "]";
                            }
                        }
                    }
                    let mut attributes_string = String::new();
                    if properties.attributes.len() > 0 {
                        attributes_string =
                            ",".to_string() + &attirbutes_to_json_object(&properties.attributes);
                    }
                    format!("{function_name}(\"{tag}\"{attributes_string}{inner_string})")
                } else {
                    let result: Vec<String> = elem
                        .children
                        .iter()
                        .map(|item| item.to_hyperscript(function_name))
                        .filter(|item| item != "")
                        .collect();
                    result.join(",")
                }
            }
            Self::Text(text) => {
                // todo: escape text & doublequotes
                let inner = text.content.to_owned();
                format!("\"{inner}\"")
            }
            Self::Html(html) => {
                // todo: change to coorect format
                let inner = html.content.to_owned();
                format!("\"{inner}\"")
            }
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

#[derive(Clone)]
pub enum AttributeValue {
    String(String),
}

#[derive(Clone)]
pub struct Attribute {
    key: String,
    value: AttributeValue,
}

impl Attribute {
    pub fn new(key: &str, value: AttributeValue) -> Attribute {
        Attribute {
            key: key.to_string(),
            value,
        }
    }

    pub fn from(key: &str, value: impl ToAttributeValue) -> Attribute {
        Attribute {
            key: key.to_string(),
            value: value.to_attribute_value(),
        }
    }
}

pub trait ToAttributeValue {
    fn to_attribute_value(&self) -> AttributeValue;
}

impl ToAttributeValue for AttributeValue {
    fn to_attribute_value(&self) -> AttributeValue {
        self.clone()
    }
}

impl ToAttributeValue for String {
    fn to_attribute_value(&self) -> AttributeValue {
        AttributeValue::String(self.to_owned())
    }
}

impl ToAttributeValue for &str {
    fn to_attribute_value(&self) -> AttributeValue {
        AttributeValue::String(self.to_string())
    }
}

#[derive(Clone)]
pub struct NodeProperties {
    tag: String,
    attributes: Vec<Attribute>,
}

// todo: macro for this
pub fn elem(selector: &str, attributes: Option<Vec<Attribute>>, children: impl ToNode) -> Node {
    let mut properties = parse_elem_properties(selector, &attributes);
    if properties.tag.len() == 0 {
        properties.tag = "div".to_string();
    }
    Node::Tag(TagNode {
        properties: Some(properties),
        children: vec![children.to_node()],
    })
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
