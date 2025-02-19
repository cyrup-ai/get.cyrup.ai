use std::collections::HashMap;

pub struct Recipe {
    pub packages: HashMap<String, Vec<String>>,
}

impl Recipe {
    pub fn new() -> Self {
        let mut packages = HashMap::new();
        packages.insert("ubuntu".to_string(), vec![
            "zsh".to_string(),
            "tmux".to_string(),
            "htop".to_string(),
        ]);
        packages.insert("amazon".to_string(), vec![
            "zsh".to_string(),
            "tmux".to_string(),
            "htop".to_string(),
        ]);
        packages.insert("macos".to_string(), vec![
            "zsh".to_string(),
            "tmux".to_string(),
            "htop".to_string(),
        ]);
        Self { packages }
    }

    pub fn get_packages(&self, platform: &str) -> Option<&Vec<String>> {
        self.packages.get(platform)
    }
}

pub struct DevToolsRecipe {
    pub packages: HashMap<String, Vec<String>>,
}

impl DevToolsRecipe {
    pub fn new() -> Self {
        let mut packages = HashMap::new();
        packages.insert("ubuntu".to_string(), vec![
            "ripgrep".to_string(),
            "fd-find".to_string(),
        ]);
        packages.insert("amazon".to_string(), vec![
            "ripgrep".to_string(),
        ]);
        packages.insert("macos".to_string(), vec![
            "ripgrep".to_string(),
            "fd".to_string(),
        ]);
        Self { packages }
    }

    pub fn get_packages(&self, platform: &str) -> Option<&Vec<String>> {
        self.packages.get(platform)
    }
}
