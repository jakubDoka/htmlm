# htmlm (for a lack of free names)

This crate implements simple html template macros without any dependencies. Use at your own risk.

# Example
```rust
let escape = htmlm::html! {
    {"<script>alert('css')</script>"}
};
assert_eq!(escape, "&lt;script&gt;alert(&#039;css&#039;)&lt;/script&gt;");

let no_escape = htmlm::html! {
    !{"<script>alert('css')</script>"}
};
assert_eq!(no_escape, "<script>alert('css')</script>");

let if_expr = htmlm::html! {
    <div>
    if let Some(y) = Some("yeh") {
        y
    } else {
        "nah"
    }
    </div>
};
assert_eq!(if_expr, "<div>yeh</div>");

let match_expr = htmlm::html! {
    <div>
    match true {
        false => { "nah" }
        trye => { "yeh" }
    }
    </div>
};
assert_eq!(match_expr, "<div>yeh</div>");

let for_expr = htmlm::html! {
    <div>
    for n in ["a", "b", "c"] {
        <div>!n</div>
    } else {
        "no content"
    }
    </div>
};
assert_eq!(for_expr, "<div><div>a</div><div>b</div><div>c</div></div>");

use std::fmt::Write;
let mut buf = String::new();
htmlm::write_html! { (buf)
    <div>"hello there"</div>
}
assert_eq!(buf, "<div>hello there</div>");
```
