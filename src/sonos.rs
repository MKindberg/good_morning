use std::{net::Ipv4Addr, str::FromStr};

pub struct Sonos {
    speakers: Vec<Speaker>,
    volume: u16,
}

impl Sonos {
    pub async fn new(ips: &[String], volume: u16) -> Sonos {
        let mut speakers = Vec::new();
        for ip in ips {
            let ip = Ipv4Addr::from_str(ip).unwrap();
            let s = Speaker::new(ip).await;
            println!("Connected to {}", s.name);
            speakers.push(s);
        }
        Sonos { speakers, volume }
    }

    pub async fn play(&self) {
        for speaker in &self.speakers {
            println!("Playing on {}", speaker.name);
            speaker
                .speaker
                .play()
                .await
                .unwrap_or_else(|e| println!("Failed to play on {}: {:?}", speaker.name, e))
        }
    }

    pub async fn set_volume(&self) {
        for speaker in &self.speakers {
            speaker.speaker.set_volume(self.volume).await.unwrap();
        }
    }

    pub async fn join(&self) {
        for speaker in self.speakers.iter().skip(1) {
            self.speakers[0]
                .speaker
                .join(&speaker.name.clone())
                .await
                .unwrap_or_else(|e| {
                    println!("Failed to join {} {}", speaker.name, e);
                    false
                });
        }
    }
}

struct Speaker {
    speaker: sonor::Speaker,
    name: String,
}

impl Speaker {
    async fn new(ip: Ipv4Addr) -> Speaker {
        let speaker = sonor::Speaker::from_ip(ip).await.unwrap().unwrap();
        let name = speaker.name().await.unwrap();
        Speaker { speaker, name }
    }
}
