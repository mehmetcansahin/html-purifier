# HTML Purifier

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](https://img.shields.io/crates/v/html-purifier.svg)](https://crates.io/crates/html-purifier)
[![Released API docs](https://docs.rs/html-purifier/badge.svg)](https://docs.rs/html-purifier)

HTML Purifier is a standard HTML filter library.

> HTML Purifier will not only remove all malicious code (better known as XSS) with a thoroughly audited, secure yet permissive whitelist, it will also make sure your documents are standards compliant, something only achievable with a comprehensive knowledge of W3C's specifications. [HTML Purifier](http://htmlpurifier.org)

## Example

```rust
use html_purifier::{purifier, Settings};

let settings = Settings {
    ..Settings::default()
};
let input = r#"<a href="/test" style="color: black;"><img src="/logo.png" onerror="javascript:;"/>Rust</a>"#;
let output = purifier(input, settings);
```

Input HTML

```html
<a href="/test" style="color: black;"
  ><img src="/logo.png" onerror="javascript:;" />Rust</a
>
```

Output HTML

```html
<a href="/test"><img src="/logo.png" />Rust</a>
```
