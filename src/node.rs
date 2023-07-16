use crate::{
    attribute::{Attribute, AttributeValue},
    event::HtmlAction,
    utils::{parse_elem_properties, HTML_VOID_ELEMENTS},
};

#[derive(Clone)]
pub struct NodeProperties {
    pub tag: String,
    pub attributes: Vec<Attribute>,
}

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
                    let mut outer_before = String::new();
                    let mut outer_after = String::new();
                    let mut attributes_strings = vec![];
                    for attribute in &properties.attributes {
                        // todo: escape quotes?
                        if attribute.key.to_lowercase() == "onclick" {
                            match &attribute.value {
                                AttributeValue::EventAction(value) => match &value.html_action {
                                    HtmlAction::Redirect { url } => {
                                        outer_before = "<a href=\"".to_string() + url + "\">";
                                        outer_after = "</a>".to_string();
                                    }
                                },
                                AttributeValue::String(_) => {}
                            }
                        } else {
                            let value = attribute.to_inline_html_item();
                            if value != "" {
                                attributes_strings.push(value);
                            }
                        }
                    }
                    let mut attributes = attributes_strings.join(" ");
                    if attributes.len() > 0 {
                        attributes = " ".to_owned() + &attributes;
                    }
                    if !HTML_VOID_ELEMENTS.contains(&tag) {
                        let inner: String =
                            elem.children.iter().map(|item| item.to_html()).collect();
                        format!("{outer_before}<{tag}{attributes}>{inner}</{tag}>{outer_after}")
                    } else {
                        format!("{outer_before}<{tag}{attributes} />{outer_after}")
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
                    let mut attributes_strings = vec![];
                    for attribute in &properties.attributes {
                        let value = attribute.to_json_object_item(function_name);
                        if value != "" {
                            attributes_strings.push(value);
                        }
                    }
                    let mut attributes_final_string = String::new();
                    if attributes_strings.len() > 0 {
                        attributes_final_string =
                            ",{".to_string() + &attributes_strings.join(",") + "}";
                    }
                    format!("{function_name}(\"{tag}\"{attributes_final_string}{inner_string})")
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
    pub properties: Option<NodeProperties>,
    pub children: Vec<Node>,
}

pub trait ToNode {
    fn to_node(&self) -> Node;
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
