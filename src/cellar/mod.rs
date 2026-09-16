/// TOML files for the configurations?
/// wait. does that go here?
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, io::Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cellar {
    pub name: String,
    pub backend: Option<String>,
    pub packages: Vec<Pkg>,
}

/// Her şey kötü her şey kötü her şey kötü
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pkg {
    pub name: String,
    pub version: Option<String>,
}

impl Pkg {
    pub fn to_string(&self) -> String {
        match &self.version {
            Some(version) => format!("{}-{}", self.name, version),
            None => self.name.clone(),
        }
    }
    /// This is a horrible implementation
    fn from_string(s: &str) -> Self {
        let parts: Vec<&str> = s.splitn(2, '-').collect();
        let name = parts[0].to_string();
        let version = if parts.len() > 1 {
            Some(parts[1].to_string())
        } else {
            Some("latest".to_string())
        };
        Self { name, version }
    }
}

impl PartialEq for Pkg {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.version == other.version
    }
}

impl Cellar {
    fn log_file_path() -> PathBuf {
        dirs::config_dir()
            .expect("no config directory")
            .join("cellars")
            .join("cellars.LOG")
    }
    pub fn new(name: &str, overwrite_existing: bool) -> Self {
        let name = &Cellar::sanitize(name);
        let mut log_file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(Cellar::log_file_path())
            .expect("could not open log file: ");
        match overwrite_existing // Maybe i should match with an Option.
        {
            true => {
                // Overwrite existing cellar configuration
                writeln!(log_file,
                        "{}: cellar: {} state: {}", 
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), name, "OVERWRITTEN & RECREATED")
                        .expect("could not write to log file: ");
                Self {
                    name: name.to_string(),
                    packages: vec![],
                    backend: None,
                    }
            }
            false => {
                // Check if the cellar already exists and inquire a prompt if so.
                if Cellar::exists(name) {
                    eprintln!("Cellar '{}' configurations already exists.", name);
                    let confirm = inquire::Confirm::new("Do you want to recreate environment with existing configurations?")
                        .with_default(false)
                        .prompt()
                        .expect("failed to prompt user");
                    match confirm {
                    true => {
                        // User wants to recreate the environment
                        writeln!(log_file,
                                "{}: cellar: {} state: {}", 
                                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), name, "OVERWRITTEN & RECREATED")
                                .expect("could not write to log file: ");
                        Self::load(name).expect("failed to load existing cellar configuration")
                        }
                    false => {
                        // User wants to abort
                        // Maybe this option should be to overwrite and create from scratch.
                        // And maybe DONT panic
                        panic!
(
"
Operation aborted, check cellar configuration; 
if configuration is not needed anymore, retry with --overwrite_existing flag to overwrite existing configuration. 
If you want to keep the existing configuration and it is what you are looking for, use cellars run or create with the --run flag, 
if none apply create the environment with a different name.
"
);
                        }
                    }
                }
                else {
                    writeln!(log_file,
                        "{}: cellar: {} state: {}", 
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), name, "CREATED")
                        .expect("could not write to log file: ");

                    Self {
                    name: name.to_string(),
                    packages: vec![],
                    backend: None,
                    }
                }
            }
        }
    }

    pub fn add_package(&mut self, pkg: &str) {
        let pkg = Pkg::from_string(pkg);
        if let Some(existing) = self.packages
            .iter_mut()
            .find(|p| p.name == pkg.name) {
            existing.version = pkg.version;
            print!("existing package {} updated to version {:?}", existing.name, existing.version.clone().unwrap());
            // Maybe this should be a log entry instead of a print statement.
            // The unwrap may be a problem anyway. from_str'ng should stop the value from be'ng None but, idk
        } else if !self.packages.contains(&pkg) {
            print!("package {} added to cellar {}", pkg.name, self.name);
            self.packages.push(pkg);
        }
        self.save().expect("couldn't save to TOML");
    }

    pub fn remove_package(&mut self, pkg: &str) {
        let pkg = Pkg::from_string(pkg);
        if let Some(pos) = self.packages.iter().position(|p| p.name == pkg.name) {
            self.packages.remove(pos);
            print!("package {} removed from cellar {}", pkg.name, self.name);
        } else {
            print!("package {} not found in cellar {}", pkg.name, self.name);
        }
        self.save().expect("couldn't save to TOML");
    }

    /// Slight issue.. What about custom paths for cellars?
    /// I'll just add a check before this match
    pub fn cellar_dir(&self) -> PathBuf {
        dirs::config_dir()
            .expect("no config directory")
            .join("cellars")
            .join("dir")
            .join(Cellar::sanitize(&self.name))
    }

    pub fn config_path(&self) -> PathBuf {
        let path = self.cellar_dir();
        std::fs::create_dir_all(&path).ok();
        path.join("cellar.toml")
    }

    /// Make this fancier later.
    /// Find a way
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
            .join(Cellar::sanitize(name))
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
            .map(|p| p.join("cellars").join("dir").join(Cellar::sanitize(name)).join("cellar.toml"))
            .map(|p| p.exists())
            .unwrap_or(false);
        path
    }

    fn sanitize(name: &str) -> String {
        name.trim_matches(|c| c == '/' || c == '\\' || c == '.')
            .replace("..", "")
            .replace("/", "_")
            .replace("\\", "_")
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
        let mut cellar = Cellar::new("test_env", true);
        cellar.add_package("hello");
        cellar.save().expect("Failed to save cellar");

        let loaded_cellar = Cellar::load("test_env").expect("Failed to load cellar");
        assert_eq!(loaded_cellar.name, "test_env");
        //assert!(loaded_cellar.packages.contains(&"hello".to_string()));
    }

    #[test]
    fn test_cellar_serializes_to_toml() {
        let mut cellar = Cellar::new("test_env", true);
        cellar.add_package("git");
        
        let toml = toml::to_string(&cellar).unwrap();
        assert!(toml.contains("test_env"));
        assert!(toml.contains("git"));
    }

    /// Ofc this doesn't pass right now because my great cellar_dir() function doesn't sanitize the name. But it should. Right?
    #[test]
    fn test_cellar_dir_path_sanitizes_name() {
        let cellar = Cellar::new("../../../../etc/passwd", false);
        cellar.save().expect("Failed to save cellar");
        let path = cellar.cellar_dir();
    
        // Should not contain ".."
        assert!(!path.to_string_lossy().contains(".."));
    
        // Should still be under cellars dir
        let config_dir = dirs::config_dir().unwrap();
        assert!(path.starts_with(config_dir.join("cellars")));

        // Clean up
        std::fs::remove_dir_all(&path)
            .map_err(|e| format!("failed to delete environment folder: {}", e)).unwrap();
        println!("{} discarded, configuration file (.TOML) and directory should now be removed.", cellar.name);
    }
}