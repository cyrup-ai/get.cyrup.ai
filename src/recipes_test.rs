#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_spec_creation() {
        let spec = PackageSpec::new("test", PackageManager::Apt);
        assert_eq!(spec.name, "test");
        assert_eq!(spec.manager_type, PackageManager::Apt);
    }

    #[test]
    fn test_recipe_validation() {
        let recipe = Recipe::new();
        assert!(recipe.validate().is_ok());

        // Test platform support
        let ubuntu_pkgs = recipe.get_packages("ubuntu").unwrap();
        assert!(!ubuntu_pkgs.is_empty());
        assert_eq!(ubuntu_pkgs[0].manager_type, PackageManager::Apt);

        let macos_pkgs = recipe.get_packages("macos").unwrap();
        assert!(!macos_pkgs.is_empty());
        assert_eq!(macos_pkgs[0].manager_type, PackageManager::Brew);

        let amazon_pkgs = recipe.get_packages("amazon").unwrap();
        assert!(!amazon_pkgs.is_empty());
        assert_eq!(amazon_pkgs[0].manager_type, PackageManager::Yum);

        // Test unsupported platform
        assert!(recipe.get_packages("windows").is_err());
    }

    #[test]
    fn test_devtools_recipe() {
        let recipe = DevToolsRecipe::new();
        assert!(recipe.validate().is_ok());

        // Check platform-specific packages
        let ubuntu = recipe.get_packages("ubuntu").unwrap();
        assert!(ubuntu.iter().any(|p| p.name == "fd-find"));

        let macos = recipe.get_packages("macos").unwrap();
        assert!(macos.iter().any(|p| p.name == "fd")); // fd-find is fd on brew

        let amazon = recipe.get_packages("amazon").unwrap();
        assert!(!amazon.iter().any(|p| p.name == "fd-find")); // Not available on Amazon Linux
    }

    #[test]
    fn test_invalid_recipe() {
        let mut recipe = Recipe::new();
        
        // Test empty package list
        recipe.packages.insert("test".to_string(), vec![]);
        assert!(recipe.validate().is_err());

        // Test inconsistent package manager
        recipe.packages.insert("test".to_string(), vec![
            PackageSpec::new("pkg1", PackageManager::Apt),
            PackageSpec::new("pkg2", PackageManager::Brew), // Wrong manager type
        ]);
        assert!(recipe.validate().is_err());

        // Test invalid package name
        recipe.packages.insert("test".to_string(), vec![
            PackageSpec::new("invalid name", PackageManager::Apt), // Contains space
        ]);
        assert!(recipe.validate().is_err());
    }
}
