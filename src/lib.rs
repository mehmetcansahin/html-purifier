//! # HTML Purifier
//!
//! HTML Purifier is a standard HTML filter library.
//!
//! > HTML Purifier will not only remove all malicious code (better known as XSS) with a thoroughly audited, secure yet permissive whitelist, it will also make sure your documents are standards compliant, something only achievable with a comprehensive knowledge of W3C's specifications. [HTML Purifier](http://htmlpurifier.org)
//!
//! ## Example
//!
//! ```
//! use html_purifier::{purifier, Settings};
//!
//! let settings = Settings {
//!     ..Settings::default()
//! };
//! let input = r#"<a href="/test" style="color: black;"><img src="/logo.png" onerror="javascript:;"/>Rust</a>"#;
//! let output = purifier(input, settings);
//! ```
//!
//! Input HTML
//!
//! ```notrust
//! <a href="/test" style="color: black;"
//!   ><img src="/logo.png" onerror="javascript:;" />Rust</a
//! >
//! ```
//!
//! Output HTML
//!
//! ```notrust
//! <a href="/test"><img src="/logo.png" />Rust</a>
//! ```

use lol_html::html_content::{Comment, Element};
use lol_html::{comments, element, rewrite_str, RewriteStrSettings};

pub struct AllowedElement {
    pub name: String,
    pub attributes: Vec<String>,
}

pub struct Settings {
    pub allowed: Vec<AllowedElement>,
    pub remove_comments: bool,
}

impl Default for Settings {
    #[inline]
    fn default() -> Self {
        Settings {
            allowed: vec![
                AllowedElement {
                    name: "div".to_string(),
                    attributes: vec![],
                },
                AllowedElement {
                    name: "b".to_string(),
                    attributes: vec![],
                },
                AllowedElement {
                    name: "strong".to_string(),
                    attributes: vec![],
                },
                AllowedElement {
                    name: "i".to_string(),
                    attributes: vec![],
                },
                AllowedElement {
                    name: "em".to_string(),
                    attributes: vec![],
                },
                AllowedElement {
                    name: "u".to_string(),
                    attributes: vec![],
                },
                AllowedElement {
                    name: "a".to_string(),
                    attributes: vec!["href".to_string(), "title".to_string()],
                },
                AllowedElement {
                    name: "ul".to_string(),
                    attributes: vec![],
                },
                AllowedElement {
                    name: "ol".to_string(),
                    attributes: vec![],
                },
                AllowedElement {
                    name: "li".to_string(),
                    attributes: vec![],
                },
                AllowedElement {
                    name: "p".to_string(),
                    attributes: vec!["style".to_string()],
                },
                AllowedElement {
                    name: "br".to_string(),
                    attributes: vec![],
                },
                AllowedElement {
                    name: "span".to_string(),
                    attributes: vec!["style".to_string()],
                },
                AllowedElement {
                    name: "img".to_string(),
                    attributes: vec![
                        "width".to_string(),
                        "height".to_string(),
                        "alt".to_string(),
                        "src".to_string(),
                    ],
                },
            ],
            remove_comments: true,
        }
    }
}

/// HTML Purifier
///
/// # Example
///
/// ```
/// use html_purifier::{purifier, Settings};
///
/// let settings = Settings {
///     ..Settings::default()
/// };
/// let input = r#"<a href="/test" style="color: black;"><img src="/logo.png" onerror="javascript:;"/>Rust</a>"#;
/// let output = purifier(input, settings);
/// ```
pub fn purifier(input: &str, settings: Settings) -> String {
    let element_handler = |el: &mut Element| {
        let find = settings.allowed.iter().find(|e| e.name.eq(&el.tag_name()));
        match find {
            Some(find) => {
                let remove_attributes = el
                    .attributes()
                    .iter()
                    .filter(|e| find.attributes.iter().any(|a| a.eq(&e.name())) == false)
                    .map(|m| m.name())
                    .collect::<Vec<String>>();
                for attr in remove_attributes {
                    el.remove_attribute(&attr);
                }
            }
            None => {
                el.remove_and_keep_content();
            }
        }
        Ok(())
    };
    let comment_handler = |c: &mut Comment| {
        if settings.remove_comments {
            c.remove();
        }
        Ok(())
    };
    let output = rewrite_str(
        input,
        RewriteStrSettings {
            element_content_handlers: vec![
                element!("*", element_handler),
                comments!("*", comment_handler),
            ],
            ..RewriteStrSettings::default()
        },
    )
    .unwrap();
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_purifier() {
        let settings = Settings {
            ..Settings::default()
        };
        let input = r#"<div style="display: block;"><span style="color: black;"><a href="/test" onclick="javascript:;"><img src="/logo.png" onerror="javascript:;"/>Rust</a></span></div>"#;
        let output = purifier(input, settings);
        assert_eq!(
            output,
            r#"<div><span style="color: black;"><a href="/test"><img src="/logo.png" />Rust</a></span></div>"#
        );
    }
    #[test]
    fn test_purifier_remove_comments() {
        let settings = Settings {
            ..Settings::default()
        };
        let input = r#"<div style="display: block;"><!--Comment 1--><span style="color: black;"><a href="/test" onclick="javascript:;"><img src="/logo.png" onerror="javascript:;"/>Rust</a></span></div>"#;
        let output = purifier(input, settings);
        assert_eq!(
            output,
            r#"<div><span style="color: black;"><a href="/test"><img src="/logo.png" />Rust</a></span></div>"#
        );
    }
    #[test]
    fn test_purifier_show_comments() {
        let settings = Settings {
            remove_comments: false,
            ..Settings::default()
        };
        let input = r#"<div style="display: block;"><span style="color: black;"><!--Comment 1--><a href="/test" onclick="javascript:;"><img src="/logo.png" onerror="javascript:;"/>Rust</a></span></div>"#;
        let output = purifier(input, settings);
        assert_eq!(
            output,
            r#"<div><span style="color: black;"><!--Comment 1--><a href="/test"><img src="/logo.png" />Rust</a></span></div>"#
        );
    }
}
