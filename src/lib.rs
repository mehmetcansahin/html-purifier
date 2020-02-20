use scraper::{Html, Selector};

pub fn purifier() {
    let html = r#"
    <ul>
        <li>Foo</li>
        <li>Bar</li>
        <li>Baz</li>
    </ul>
"#;

    let fragment = Html::parse_fragment(html);
    for node in fragment.tree {
        
    }
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
