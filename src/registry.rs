#[derive (Debug, Serialize, Deserialize)]
pub struct Registry {
    pub cellars: Vec<CellarRecord>,
    path: PathBuf,
}

#[derive (Debug, Serialize, Deserialize)]
pub struct CellarRecord {
    pub name: String,
    pub path: String,
    pub backend: String,
    pub state: CellarState
}

#[derive (Debug, Serialize, Deserialize)]
pub enum CellarState {
    Active,
    Killed,
    Discarded
}

#[derive (Debug, Serialize, Deserialize)]
pub enum RuntimeState {
    Active,
    Inactive,
    Unknown
}

impl Registry {
    pub fn load()  {} 
    pub fn save()  {}
    pub fn update() {}
}