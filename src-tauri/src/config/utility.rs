use std::path::{Path, PathBuf};

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

    pub fn read_xml_file<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        Ok(content)
    }

    pub fn get_app_base_path() -> PathBuf {
        let app_path = std::env::current_exe().expect("Failed to get current exe path");
        app_path
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .expect("Failed to get parent path")
            .to_path_buf()
    }

    pub fn get_config_path() -> PathBuf {
        let base_path = Self::get_app_base_path();
        base_path.join("config.json")
    }

    pub fn get_tally_request_path(request_type: &str) -> PathBuf {
        let base_path = Self::get_app_base_path();
        base_path
            .join("src")
            .join(format!("tally_request/{}.xml", request_type))
    }
}
