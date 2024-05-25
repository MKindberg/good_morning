use crate::config::SonosAlarm;
use sonor::URN;
use std::{net::Ipv4Addr, str::FromStr};

pub struct Sonos {
    speakers: Vec<Speaker>,
    volume: u16,
    alarm_data: SonosAlarm,
}

impl Sonos {
    pub async fn new(ips: &[String], volume: u16, alarm_data: SonosAlarm) -> Sonos {
        let mut speakers = Vec::new();
        for ip in ips {
            let ip = Ipv4Addr::from_str(ip).expect("Failed to parse sonos IP");
            let s = Speaker::new(ip).await;
            println!("Connected to {}", s.name);
            speakers.push(s);
        }
        Sonos {
            speakers,
            volume,
            alarm_data,
        }
    }

    pub async fn set_alarm(&self, time: &str) {
        let service = URN::service("schemas-upnp-org", "AlarmClock", 1);
        let aaction = "UpdateAlarm";
        let args = format!(
            "<ID>375</ID>
<StartLocalTime>{}:00</StartLocalTime>
<Duration></Duration>
<Recurrence>ONCE</Recurrence>
<Enabled>1</Enabled>
<RoomUUID>{}</RoomUUID>
<ProgramURI>{}</ProgramURI>
<ProgramMetaData>{}</ProgramMetaData>
<PlayMode>SHUFFLE</PlayMode>
<Volume>{}</Volume>
<IncludeLinkedZones>1</IncludeLinkedZones>
",
            time,
            self.alarm_data.room_uuid,
            self.alarm_data.program_uri,
            self.alarm_data.program_meta_data,
            self.volume
        );
        self.speakers[0]
            .speaker
            .action(&service, aaction, &args)
            .await
            .expect("Failed to set alarm");
    }
    pub async fn unset_alarm(&self) {
        let service = URN::service("schemas-upnp-org", "AlarmClock", 1);
        let aaction = "UpdateAlarm";
        let args = format!(
            "<ID>375</ID>
<StartLocalTime>11:00:00</StartLocalTime>
<Duration></Duration>
<Recurrence>ONCE</Recurrence>
<Enabled>0</Enabled>
<RoomUUID>{}</RoomUUID>
<ProgramURI>{}</ProgramURI>
<ProgramMetaData>{}</ProgramMetaData>
<PlayMode>SHUFFLE</PlayMode>
<Volume>{}</Volume>
<IncludeLinkedZones>1</IncludeLinkedZones>
",
            self.alarm_data.room_uuid,
            self.alarm_data.program_uri,
            self.alarm_data.program_meta_data,
            self.volume
        );
        self.speakers[0]
            .speaker
            .action(&service, aaction, &args)
            .await
            .expect("Failed to set alarm");
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
            speaker
                .speaker
                .set_volume(self.volume)
                .await
                .expect("Failed to set volume");
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
