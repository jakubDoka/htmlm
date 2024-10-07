#![doc = include_str!("../README.md")]

pub use htmlm_macro::html;
pub use htmlm_macro::write_html;

pub struct HtmlEscaped<T>(pub T);

impl<T: std::fmt::Display> std::fmt::Display for HtmlEscaped<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use core::fmt::Write;

        struct Escaper<'a>(std::fmt::Formatter<'a>);

        impl<'a> std::fmt::Write for Escaper<'a> {
            fn write_str(&mut self, s: &str) -> std::fmt::Result {
                let mut prev_pos = 0;
                for (pos, c) in s.bytes().enumerate() {
                    let escape = match c {
                        b'<' => "&lt;",
                        b'>' => "&gt;",
                        b'\"' => "&quot;",
                        b'\'' => "&#039;",
                        b'&' => "&amp;",
                        _ => continue,
                    };
                    if prev_pos < pos {
                        self.0.write_str(&s[prev_pos..pos])?;
                    }
                    self.0.write_str(escape)?;
                    prev_pos = pos + 1;
                }
                if prev_pos < s.len() {
                    self.0.write_str(&s[prev_pos..])?;
                }

                Ok(())
            }
        }

        let ef = unsafe { std::mem::transmute::<&mut std::fmt::Formatter<'_>, &mut Escaper>(f) };
        write!(ef, "{}", self.0)
    }
}
