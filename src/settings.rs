use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
pub struct Settings {
    pub port: String,
    pub config_file: String,
}

impl Settings {
    pub fn new() -> Self {
        let config = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
        serde_yaml::from_str(&config).unwrap()
    }
}
