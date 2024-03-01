use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
pub struct Config {
    pub sonos: Sonos,
    pub openhab: Vec<Openhab>,
}

impl Config {
    pub fn new(config_file: &str) -> Config {
        let config = std::fs::read_to_string(config_file).expect("Could not read config file");
        serde_yaml::from_str(&config).expect("Could not parse config file")
    }
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Sonos {
    pub ips: Vec<String>,
    pub volume: u16,
    pub alarm: SonosAlarm,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct SonosAlarm {
    pub room_uuid: String,
    pub program_uri: String,
    pub program_meta_data: String,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Openhab {
    pub name: String,
    pub value: String,
}
