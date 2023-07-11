use std::{collections::HashMap, vec};

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
    properties: Option<NodeProperties>,
    children: Vec<Node>,
}

pub trait ToNode {
    fn to_node(&self) -> Node;
}

#[derive(Clone)]
pub enum AttributeValue {
    String(String),
    EventAction(EventAction),
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

    // todo: consider escaping doublequotes
    pub fn to_inline_html_item(&self) -> String {
        let key = &self.key;
        match &self.value {
            AttributeValue::String(value) => {
                format!("{key}=\"{value}\"")
            }
            AttributeValue::EventAction(_) => String::from(""),
        }
    }

    // todo: use proper json
    pub fn to_json_object_item(&self, function_name: &str) -> String {
        let key = &self.key;
        match &self.value {
            AttributeValue::String(value) => {
                format!("\"{key}\":\"{value}\"")
            }
            AttributeValue::EventAction(value) => {
                let attribute_value = value.hyperscript_action.to_hyperscript(function_name);
                format!("\"{key}\":\"{attribute_value}\"")
            }
        }
    }
}

#[derive(Clone)]
pub struct EventAction {
    hyperscript_action: HyperscriptAction,
    html_action: HtmlAction,
}

impl EventAction {
    pub fn ajax_default(url: &str, ajax_options: Option<AjaxRequestOptions>) -> Self {
        Self {
            hyperscript_action: HyperscriptAction::ajax_default(url, ajax_options),
            html_action: HtmlAction::redirect(url),
        }
    }
}

#[derive(Clone)]
pub enum HyperscriptAction {
    AjaxRequest {
        url: String,
        options: Option<AjaxRequestOptions>,
    },
}

impl HyperscriptAction {
    pub fn ajax_default(url: &str, options: Option<AjaxRequestOptions>) -> Self {
        Self::AjaxRequest {
            url: url.to_owned(),
            options,
        }
    }

    pub fn to_hyperscript(&self, function_name: &str) -> String {
        match self {
            Self::AjaxRequest { url, options } => {
                // todo: escape quotes
                let mut options_strings = vec![];
                if let Some(options) = options {
                    if options.method != "" {
                        let method = &options.method;
                        options_strings.push(format!("method:\"{method}\""));
                    } else {
                        options_strings.push(format!("method:\"GET\""));
                    }
                    if options.params.len() > 0 {
                        // todo: escape quotes
                        let mut params_strings = vec![];
                        for (key, value) in &options.params {
                            params_strings.push(format!("{key}:\"{value}\""));
                        }
                        let params_strings = params_strings.join(",");
                        options_strings.push("{".to_string() + &params_strings + "}");
                    }
                    if options.body != "" {
                        let body = &options.body;
                        options_strings.push(format!("body:\"{body}\""));
                    }
                }
                let mut arguments_string = format!("url:\"{url}\"");
                if options_strings.len() > 0 {
                    let options_strings = options_strings.join(",");
                    arguments_string = arguments_string + "," + &options_strings;
                }
                "function(){".to_string() + function_name + ".request({" + &arguments_string + "})}"
            }
        }
    }
}

#[derive(Clone)]
pub enum HtmlAction {
    Redirect {
        // behavior: nodes with this as onclick action get a parent <a href="url"> tag (only for html output)
        url: String,
    },
}

impl HtmlAction {
    pub fn redirect(url: &str) -> Self {
        Self::Redirect {
            url: url.to_owned(),
        }
    }
}

#[derive(Clone)]
pub struct AjaxRequestOptions {
    method: String,
    params: HashMap<String, String>,
    body: String,
}

impl AjaxRequestOptions {
    pub fn new(
        method: Option<&str>,
        params: Option<HashMap<String, String>>,
        body: Option<&str>,
    ) -> Self {
        Self {
            method: method.unwrap_or_default().to_string(),
            params: params.unwrap_or_default(),
            body: body.unwrap_or_default().to_string(),
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

impl ToAttributeValue for EventAction {
    fn to_attribute_value(&self) -> AttributeValue {
        AttributeValue::EventAction(self.clone())
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
