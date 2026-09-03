/// TOML files for the configurations?
/// wait. does that go here?
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cellar {
    pub name: String,
    pub packages: Vec<String>,
}

impl Cellar {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            packages: vec![],
        }
    }

    pub fn add_package(&mut self, pkg: &str) {
        if !self.packages.contains(&pkg.to_string()) {
            self.packages.push(pkg.to_string());
        }
    }

    pub fn cellar_dir(&self) -> PathBuf {
        dirs::config_dir()
            .expect("no config directory")
            .join("cellars")
            .join("dir")
            .join(&self.name)
    }

    pub fn config_path(&self) -> PathBuf {
        let path = self.cellar_dir();
        std::fs::create_dir_all(&path).ok();
        path.join("cellar.toml")
    }

    pub fn save(&self) -> Result<(), String> {
        let path = self.config_path();
        let toml = toml::to_string_pretty(self)
            .map_err(|e| format!("serialize failed: {}", e))?;
        std::fs::write(&path, toml)
            .map_err(|e| format!("save failed: {}", e))?;
        Ok(())
    }

    pub fn load(name: &str) -> Result<Self, String> {
        let path = dirs::config_dir()
            .expect("no config directory")
            .join("cellars")
            .join("dir")
            .join(name)
            .join("cellar.toml");

        let contents = std::fs::read_to_string(&path)
            .map_err(|e| format!("load failed: {}", e))?;
        let cellar: Cellar = toml::from_str(&contents)
            .map_err(|e| format!("parse failed: {}", e))?;

        Ok(cellar)
    }

    /// Returns true if the cellar TOML file exists. Doesn't check the shell.nix file or the TOML file's content.
    pub fn exists(name: &str) -> bool {
        let path = dirs::config_dir()
            .map(|p| p.join("cellars").join("dir").join(name).join("cellar.toml"))
            .map(|p| p.exists())
            .unwrap_or(false);
        path
    }
}

mod tests {
    use super::*;

    #[test]
    fn new_cellar_initializes_correctly() {

    }

    #[test]
    fn cellar_package_list_stored_correctly() {

    }


    #[test]
    fn cellar_can_save_and_load() {
        let mut cellar = Cellar::new("test_env");
        cellar.add_package("hello");
        cellar.save().expect("Failed to save cellar");

        let loaded_cellar = Cellar::load("test_env").expect("Failed to load cellar");
        assert_eq!(loaded_cellar.name, "test");
        assert!(loaded_cellar.packages.contains(&"hello".to_string()));
    }

    #[test]
    fn test_cellar_serializes_to_toml() {
        let mut cellar = Cellar::new("test_env");
        cellar.add_package("git");
        
        let toml = toml::to_string(&cellar).unwrap();
        assert!(toml.contains("test_env"));
        assert!(toml.contains("git"));
    }

    /// Ofc this doesn't pass right now because my great cellar_dir() function doesn't sanitize the name. But it should. Right?
    #[test]
    fn test_cellar_dir_path_sanitizes_name() {
        let cellar = Cellar::new("../../../../etc/passwd");
        let path = cellar.cellar_dir();
    
        // Should not contain ".."
        assert!(!path.to_string_lossy().contains(".."));
    
        // Should still be under cellars dir
        let config_dir = dirs::config_dir().unwrap();
        assert!(path.starts_with(config_dir.join("cellars")));
    }
}