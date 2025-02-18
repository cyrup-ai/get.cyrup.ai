use std::collections::HashMap;

pub const RUSTFMT_CONFIG: &str = r#"
max_width = 100
tab_spaces = 4
edition = "2021"
use_small_heuristics = "Max"
imports_granularity = "Module"
group_imports = "StdExternalCrate"
"#;

pub const CARGO_CONFIG: &str = r#"
[build]
rustflags = ["-C", "target-cpu=native"]

[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/opt/llvm/bin/ld64.lld"]

[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/opt/homebrew/opt/llvm/bin/ld64.lld"]

[registries.crates-io]
protocol = "sparse"
"#;

pub struct Config {
    keys: HashMap<String, Option<String>>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    pub fn key(mut self, name: &str) -> Self {
        self.keys.insert(name.to_string(), std::env::var(name).ok());
        self
    }

    pub fn build(self) -> HashMap<String, String> {
        self.keys.into_iter()
            .filter_map(|(k, v)| v.map(|v| (k, v)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        std::env::set_var("TEST_KEY", "test_value");
        
        let config = Config::new()
            .key("TEST_KEY")
            .key("NONEXISTENT_KEY")
            .build();
        
        assert_eq!(config.get("TEST_KEY"), Some(&"test_value".to_string()));
        assert_eq!(config.get("NONEXISTENT_KEY"), None);
    }
}
