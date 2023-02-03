use config::Config;
use reqwest::{ self };
use std::{ fs::File, io::Read, path::Path };

mod structs;
mod config;
mod errors;

use structs::{ Response, DiscordPost };
use errors::{ Error, RequestError };

fn main() {
    let result = run();

    if let Err(err) = result {
        println!("{}", err);
        handle_error(err).unwrap();
    }
}

fn run() -> Result<(), Error> {
    let config = Config::load()?;

    let data = reqwest::blocking
        ::get(
            if cfg!(debug_assertions) {
                config.dev_alert_url // TODO: Add mock api
            } else {
                config.prod_alert_url
            }
        )?
        .json::<Vec<Response>>()?;

    if data.len() > 0 {
        let path = "./history.txt";
        let history: Vec<Response> = if Path::new(path).is_file() {
            let mut file = File::open(path)?;
            let mut data = String::new();

            file.read_to_string(&mut data)?;

            serde_json::from_str(&data)?
        } else {
            File::create(path)?;

            Vec::new()
        };

        let mut save: Vec<Response> = Vec::with_capacity(history.len() + data.len());

        for msg in data {
            if history.contains(&msg) {
                if !save.contains(&msg) {
                    save.push(msg.clone());
                }
                continue;
            }

            send_discord_message(
                if cfg!(debug_assertions) {
                    &config.dev_webhook // debug
                } else {
                    &config.prod_webhook // release
                },
                format!("@everyone {}", msg.title.rendered)
            )?;

            save.push(msg);
        }

        let mut file = File::create(path)?;
        serde_json::to_writer(&mut file, &save)?;
    }

    Ok(())
}

fn send_discord_message(url: &str, content: String) -> Result<(), Error> {
    let client = reqwest::blocking::Client::new();

    let res = client.post(url).json(&(DiscordPost { content })).send()?;

    if !res.status().is_success() {
        return Err(
            Error::RequestError(RequestError {
                code: res.status(),
                url: res.url().to_string(),
                msg: res.text()?.to_string(),
            })
        );
    }

    Ok(())
}

fn handle_error(err: Error) -> Result<(), Error> {
    let config = Config::load()?;

    send_discord_message(&config.error_webhook, format!("{}", err))?;

    Ok(())
}