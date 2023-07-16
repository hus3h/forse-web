use std::collections::HashMap;

#[derive(Clone)]
pub struct EventAction {
    pub hyperscript_action: HyperscriptAction,
    pub html_action: HtmlAction,
}

impl EventAction {
    pub fn ajax_default(url: &str, ajax_options: Option<Vec<AjaxRequestOption>>) -> Self {
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
        options: Vec<AjaxRequestOption>,
    },
}

impl HyperscriptAction {
    pub fn ajax_default(url: &str, options: Option<Vec<AjaxRequestOption>>) -> Self {
        Self::AjaxRequest {
            url: url.to_owned(),
            options: options.unwrap_or_default(),
        }
    }

    pub fn to_hyperscript(&self, function_name: &str) -> String {
        match self {
            Self::AjaxRequest { url, options } => {
                // todo: escape quotes
                let mut options_strings = vec![];
                let mut request_method = "";
                for option in options {
                    match option {
                        AjaxRequestOption::Method(value) => {
                            request_method = value;
                        }
                        AjaxRequestOption::Params(value) => {
                            // todo: escape quotes
                            let mut params_strings = vec![];
                            for (key, value) in value {
                                params_strings.push(format!("{key}:\"{value}\""));
                            }
                            let params_strings = params_strings.join(",");
                            options_strings.push("params:{".to_string() + &params_strings + "}");
                        }
                        AjaxRequestOption::Headers(value) => {
                            // todo: escape quotes
                            let mut params_strings = vec![];
                            for (key, value) in value {
                                params_strings.push(format!("{key}:\"{value}\""));
                            }
                            let params_strings = params_strings.join(",");
                            options_strings.push("headers:{".to_string() + &params_strings + "}");
                        }
                        AjaxRequestOption::User(value) => {
                            options_strings.push(format!("user:\"{value}\""));
                        }
                        AjaxRequestOption::Password(value) => {
                            options_strings.push(format!("password:\"{value}\""));
                        }
                        AjaxRequestOption::Body(value) => {
                            options_strings.push(format!("body:\"{value}\""));
                        }
                        AjaxRequestOption::WithCredentials(value) => {
                            let value_string = {
                                if *value {
                                    "true"
                                } else {
                                    "false"
                                }
                            };
                            options_strings.push(format!("withCredentials:{value_string}"));
                        }
                    }
                }
                if request_method == "" {
                    request_method = "GET";
                }
                options_strings.push(format!("method:\"{request_method}\""));
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

// https://mithril.js.org/request.html
#[derive(Clone)]
pub enum AjaxRequestOption {
    Method(String),
    Params(HashMap<String, String>),
    Body(String),
    User(String),
    Password(String),
    WithCredentials(bool),
    Headers(HashMap<String, String>),
}
