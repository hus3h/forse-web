use fancy_regex::Regex;
use std::collections::HashMap;

use crate::NodeProperties;

pub fn parse_html_selector(selector: &str) -> NodeProperties {
    let mut tag = String::new();
    let mut attributes = HashMap::new();
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
                '#' => _ = attributes.insert("id".to_string(), match_value),
                '[' => {
                    // todo: support empty attributes
                    let regex = Regex::new(r#"\[[\s]*(.*)=(.*)[\s]*\]"#).unwrap();
                    let result = regex.captures(&selector_match).unwrap();

                    if let Some(result) = result {
                        attributes.insert(
                            result.get(1).unwrap().as_str().trim().to_owned(),
                            result.get(2).unwrap().as_str().trim().to_owned(),
                        );
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
    if classes.len() > 0 {
        attributes.insert("class".to_string(), classes.join(" "));
    }

    NodeProperties { tag, attributes }
}

// todo: consider escaping doublequotes
pub fn attirbutes_to_inline_html(attributes: &HashMap<String, String>) -> String {
    let mut result = vec![];
    for (key, value) in attributes {
        result.push(format!("{key}=\"{value}\""));
    }
    result.join(" ")
}
