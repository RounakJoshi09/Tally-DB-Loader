pub struct Utility {}

impl Utility {
    pub fn escape_html(unsafe_str: &str) -> String {
        let mut escaped = String::new();
        for c in unsafe_str.chars() {
            match c {
                '&' => escaped.push_str("&amp;"),
                '<' => escaped.push_str("&lt;"),
                '>' => escaped.push_str("&gt;"),
                '"' => escaped.push_str("&quot;"),
                '\'' => escaped.push_str("&apos;"),
                _ => escaped.push(c),
            }
        }
        escaped
    }
}
