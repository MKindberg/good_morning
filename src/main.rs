#![feature(async_closure)]
mod config;
mod openhab;
mod settings;
mod sonos;

use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use clokwerk::{AsyncScheduler, Job, TimeUnits};
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    let settings = settings::Settings::new();

    let listener =
        TcpListener::bind(&format!("0.0.0.0:{}", settings.port)).expect("Could not bind to port");
    println!("Listening on port {}", settings.port);

    let mut thread_handle: Option<JoinHandle<()>> = None;
    for stream in listener.incoming() {
        let config = config::Config::new(&settings.config_file);
        let speakers = sonos::Sonos::new(
            config.sonos.ips.as_slice(),
            config.sonos.volume,
            config.sonos.alarm,
        )
        .await;
        if let Some(handle) = &thread_handle {
            if !handle.is_finished() {
                println!("Aborting current alarm");
                handle.abort();
                speakers.unset_alarm().await;
            }
        }
        let stream = stream.unwrap();

        if let Some(time) = parse_http(stream) {
            println!("Schedule new alarm for {}", &time);

            speakers.join().await;
            speakers.set_alarm(&time).await;
            let mut scheduler = AsyncScheduler::new();
            scheduler
                .every(1.day())
                .at(&time)
                .run(async || {
                    trigger_alarm().await;
                })
                .once();
            thread_handle = Some(tokio::spawn(async move {
                loop {
                    scheduler.run_pending().await;
                    tokio::time::sleep(Duration::from_millis(60 * 1000)).await;
                }
            }));
        }
    }
}

async fn trigger_alarm() {
    println!("Alarm triggered");
    let settings = settings::Settings::new();
    let config = config::Config::new(&settings.config_file);
    let oh_items = config
        .openhab
        .iter()
        .map(|i| openhab::Item::new(&i.name, &i.value));

    // speakers.set_volume().await;
    // speakers.play().await;

    thread::sleep(Duration::from_secs(60 * 2));

    for item in oh_items {
        item.trigger().await;
    }

    println!("Alarm finished");
}

fn parse_http(mut stream: TcpStream) -> Option<String> {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    for req in http_request {
        if !req.starts_with("GET") {
            continue;
        }
        if req.find("time=").is_some() {
            let time = req.split_once('=').unwrap().1.split_once(' ').unwrap().0;
            return if time.len() > 0 {
                Some(time.to_string())
            } else {
                None
            };
        }
    }
    None
}
