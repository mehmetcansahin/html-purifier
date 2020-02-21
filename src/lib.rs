use lol_html::html_content::Element;
use lol_html::{element, rewrite_str, RewriteStrSettings};

// https://docs.rs/lol_html/0.1.0/lol_html/html_content/struct.Element.html#method.remove
pub fn purifier() {
    let input = r#"<div><span><!-- 42 --></span></div>"#;

    let div_element_handler = |el: &mut Element| {
        el.remove_and_keep_content();
        Ok(())
    };

    let output = rewrite_str(
        input,
        RewriteStrSettings {
            element_content_handlers: vec![
                element!("div", div_element_handler),
                element!("span", div_element_handler),
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
