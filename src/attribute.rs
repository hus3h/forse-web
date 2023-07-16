use crate::event::EventAction;

#[derive(Clone)]
pub enum AttributeValue {
    String(String),
    EventAction(EventAction),
}

#[derive(Clone)]
pub struct Attribute {
    pub key: String,
    pub value: AttributeValue,
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
