use config::Config;
use dotenvy::dotenv;
use reqwest;
use std::{collections::HashSet, thread, time::Duration};

mod config;
mod errors;
mod structs;

use errors::{Error, RequestError};
use structs::{DiscordPost, Response};

fn main() {
    dotenv().unwrap();
    let config = Config::load().expect("Could not load config");
    let mut history = HashSet::new();
    let mut num_errors = 0;

    loop {
        let result = run(&mut history, &config);

        if let Err(err) = result {
            println!("{}", err);
            if num_errors >= 2 {
                handle_error(err).unwrap();
            } else {
                num_errors += 1
            }
        } else {
            num_errors = 0;
        }

        thread::sleep(Duration::from_secs(60 * 3))
    }
}

fn run(history: &mut HashSet<Response>, config: &Config) -> Result<(), Error> {
    let data = reqwest::blocking::get(&config.alert_url)?.json::<Vec<Response>>()?;

    for msg in data.iter().filter(|item| !history.contains(item)) {
        if msg.post_meta.site_id_list.contains(&"41".to_string()) {
            send_discord_message(&config.webhook, format!("@everyone {}", msg.title.rendered))?;
        } else {
            send_discord_message(
                &config.webhook,
                format!("Not for YHS:\n{}", msg.title.rendered),
            )?;
        }
        send_discord_message(&config.error_webhook, format!("{:#?}", msg))?;
    }

    let removed = history
        .iter()
        .filter(|item| !data.contains(item))
        .collect::<Vec<_>>();

    if !removed.is_empty() {
        send_discord_message(&config.error_webhook, format!("Removed: {:#?}", removed))?;
    }

    history.clear();

    for msg in data {
        history.insert(msg);
    }

    Ok(())
}

fn send_discord_message(url: &str, content: String) -> Result<(), Error> {
    let client = reqwest::blocking::Client::new();

    let res = client.post(url).json(&(DiscordPost { content })).send()?;

    if !res.status().is_success() {
        return Err(Error::RequestError(RequestError {
            code: res.status(),
            url: res.url().to_string(),
            msg: res.text()?.to_string(),
        }));
    }

    Ok(())
}

fn handle_error(err: Error) -> Result<(), Error> {
    let config = Config::load()?;

    send_discord_message(&config.error_webhook, format!("{}", err))?;

    Ok(())
}
