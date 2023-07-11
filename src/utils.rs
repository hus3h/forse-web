use fancy_regex::Regex;

use crate::{Attribute, AttributeValue, NodeProperties};

// https://github.com/MithrilJS/mithril.js/blob/71ce364c54bc501ce4a02f34e8d60271fe4b4905/render/hyperscript.js#L7
pub fn parse_elem_properties(
    selector: &str,
    attributes: &Option<Vec<Attribute>>,
) -> NodeProperties {
    let mut tag = String::new();
    let mut node_attributes = Vec::new();
    let mut classes = vec![];

    let regex = Regex::new(
        r#"(?:(^|#|\.)([^#\.\[\]]+))|(\[(.+?)(?:\s*=\s*("|'|)((?:\\["'\]]|.)*?)\5)?\])"#,
    )
    .unwrap();

    for selector_match in regex.find_iter(selector) {
        let selector_match = selector_match.unwrap().as_str().trim().to_string();

        if selector_match.len() > 0 {
            let match_type = selector_match.chars().nth(0).unwrap();
            let match_value: String = selector_match.chars().skip(1).collect();

            match match_type {
                '.' => classes.push(match_value),
                '#' => _ = node_attributes.push(Attribute::from("id", match_value)),
                '[' => {
                    // todo: can [x] values contain escaped equal signs?
                    if match_value.contains('=') {
                        let regex = Regex::new(r#"\[[\s]*(.*)=(.*)[\s]*\]"#).unwrap();
                        let result = regex.captures(&selector_match).unwrap();

                        if let Some(result) = result {
                            node_attributes.push(Attribute::from(
                                result.get(1).unwrap().as_str().trim(),
                                result.get(2).unwrap().as_str().trim(),
                            ));
                        }
                    } else {
                        // todo: make sure the value should be like this
                        let regex = Regex::new(r#"^\[(.*)\]$"#).unwrap();
                        let result = regex
                            .captures(&selector_match)
                            .unwrap()
                            .unwrap()
                            .get(1)
                            .unwrap()
                            .as_str();

                        node_attributes.push(Attribute::from(result, ""));
                    }

                    /*
                    let parts: Vec<String> = match_value
                        .split("=")
                        .map(|item| item.to_string())
                        .collect();
                    let key = parts.get(0).unwrap();
                    let other_parts: Vec<String> =
                        parts.iter().skip(1).map(|item| item.to_string()).collect();
                    let mut value = other_parts.join("=");
                    value.pop();

                    attributes.insert(key.to_owned(), value);
                    */
                }
                _ => tag = selector_match,
            }
        }
    }

    let tag = tag.to_string();

    if let Some(attributes) = attributes {
        for attribute in attributes {
            let key = &attribute.key;
            match &attribute.value {
                AttributeValue::String(value) => {
                    if key == "class" || key == "className" {
                        classes.push(value.to_owned());
                    } else {
                        node_attributes.push(Attribute::new(
                            &key,
                            AttributeValue::String(value.to_owned()),
                        ));
                    }
                }
                _ => {
                    node_attributes.push(Attribute::new(&key, (&attribute.value).to_owned()));
                }
            }
        }
    }

    if classes.len() > 0 {
        node_attributes.push(Attribute::from("class", classes.join(" ")));
    }

    NodeProperties {
        tag,
        attributes: node_attributes,
    }
}
