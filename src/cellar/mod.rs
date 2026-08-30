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

    pub fn exists(name: &str) -> bool {
        let path = dirs::config_dir()
            .map(|p| p.join("cellars").join("dir").join(name).join("cellar.toml"))
            .map(|p| p.exists())
            .unwrap_or(false);
        path
    }
}