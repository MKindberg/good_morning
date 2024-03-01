use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
pub struct Settings {
    pub port: String,
    pub config_file: String,
}

impl Settings {
    pub fn new() -> Self {
        let config = std::fs::read_to_string(
            std::env::args()
                .nth(1)
                .expect("First argument must be a settings file"),
        )
        .expect("Cound not read settings file");
        serde_yaml::from_str(&config).expect("Failed to parse settings")
    }
}
