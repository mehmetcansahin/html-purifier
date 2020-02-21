use lol_html::{element, rewrite_str, RewriteStrSettings};
// https://docs.rs/lol_html/0.1.0/lol_html/html_content/struct.Element.html#method.remove
pub fn purifier() {
    let html = rewrite_str(
        r#"<div><span><!-- 42 --></span></div>"#,
        RewriteStrSettings {
            element_content_handlers: vec![element!("div", |el| {
                el.remove_and_keep_content();

                Ok(())
            })],
            ..RewriteStrSettings::default()
        },
    )
    .unwrap();
    println!("{}", html);
    assert_eq!(html, r#"<span><!-- 42 --></span>"#);
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
