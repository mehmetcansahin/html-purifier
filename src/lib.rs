use lol_html::html_content::Element;
use lol_html::{element, rewrite_str, RewriteStrSettings};

pub struct AllowedElement {
    pub name: String,
    pub attribute: Vec<String>,
}

pub struct HTML {
    pub allowed: Vec<AllowedElement>,
}

pub struct Settings {
    pub default: HTML,
}

impl Default for Settings {
    #[inline]
    fn default() -> Self {
        Settings {
            default: HTML {
                allowed: vec![
                    AllowedElement {
                        name: "div".to_string(),
                        attribute: vec![],
                    },
                    AllowedElement {
                        name: "b".to_string(),
                        attribute: vec![],
                    },
                    AllowedElement {
                        name: "strong".to_string(),
                        attribute: vec![],
                    },
                    AllowedElement {
                        name: "i".to_string(),
                        attribute: vec![],
                    },
                    AllowedElement {
                        name: "em".to_string(),
                        attribute: vec![],
                    },
                    AllowedElement {
                        name: "u".to_string(),
                        attribute: vec![],
                    },
                    AllowedElement {
                        name: "a[href|title]".to_string(),
                        attribute: vec!["href".to_string(), "title".to_string()],
                    },
                    AllowedElement {
                        name: "ul".to_string(),
                        attribute: vec![],
                    },
                    AllowedElement {
                        name: "ol".to_string(),
                        attribute: vec![],
                    },
                    AllowedElement {
                        name: "li".to_string(),
                        attribute: vec![],
                    },
                    AllowedElement {
                        name: "p".to_string(),
                        attribute: vec!["style".to_string()],
                    },
                    AllowedElement {
                        name: "br".to_string(),
                        attribute: vec![],
                    },
                    AllowedElement {
                        name: "span".to_string(),
                        attribute: vec!["style".to_string()],
                    },
                    AllowedElement {
                        name: "img".to_string(),
                        attribute: vec![
                            "width".to_string(),
                            "height".to_string(),
                            "alt".to_string(),
                            "src".to_string(),
                        ],
                    },
                ],
            },
        }
    }
}

// https://docs.rs/lol_html/0.1.0/lol_html/html_content/struct.Element.html#method.remove
pub fn purifier() {
    let settings = Settings {
        ..Settings::default()
    };

    let input = r#"<div src="asd"><span style=""><!-- 42 --></span></div>"#;

    let html_element_handler = |el: &mut Element| {
        let find_el = settings
            .default
            .allowed
            .iter()
            .find(|x| x.name.eq(&el.tag_name()));
        match find_el {
            Some(finded_el) => {
                let mut remove_attr = vec![];
                for attr in el.attributes() {
                    if finded_el.attribute.iter().any(|x| x.eq(&attr.name())) == false {
                        remove_attr.push(attr.name());
                    }
                }
                for attr in remove_attr {
                    el.remove_attribute(&attr);
                }
            }
            None => {
                el.remove_and_keep_content();
            }
        }
        Ok(())
    };

    let output = rewrite_str(
        input,
        RewriteStrSettings {
            element_content_handlers: vec![
                element!("div", html_element_handler),
                element!("span", html_element_handler),
            ],
            ..RewriteStrSettings::default()
        },
    )
    .unwrap();
    println!("{}", output);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        purifier();
        assert_eq!(2 + 2, 4);
    }
}
