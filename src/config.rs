use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
pub struct Config {
    pub items: Items,
}

impl Config {
    pub fn new() -> Config {
        let config = std::fs::read_to_string(
            std::env::var("HOME").unwrap() + "/.config/good_morning/items.yaml",
        )
        .unwrap();
        serde_yaml::from_str(&config).unwrap()
    }
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Items {
    pub sonos: Sonos,
    pub openhab: Vec<Openhab>,
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
